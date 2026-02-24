use core::{
    cell::RefCell,
    char::{REPLACEMENT_CHARACTER, decode_utf16},
    fmt, ptr,
};

use super::memory_map::MemoryMap;

pub type EFIHandle = *mut u8;

#[repr(C)]
#[derive(Debug)]
pub struct EFIGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

pub trait Guid {
    fn guid() -> EFIGuid;
}

const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EFIGuid = EFIGuid {
    data1: 0x9042a9de,
    data2: 0x23dc,
    data3: 0x4a38,
    data4: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct EFIStatus(usize);

#[repr(C)]
#[derive(Debug)]
pub struct EFITableHeader {
    pub signature: u64,
    pub revision: u32,
    pub size: u32,
    pub crc32: u32,
    _reserved: u32,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct CChar(*const u16);

impl CChar {
    fn len(&self) -> usize {
        let mut offset = 0;
        while unsafe { *self.0.add(offset) } != 0 {
            offset += 1;
        }
        offset as _
    }

    fn as_slice(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.0, self.len()) }
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        decode_utf16(self.as_slice().iter().copied()).map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
    }
}

impl fmt::Display for CChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.chars().for_each(|c| {
            _ = write!(f, "{c}");
        });
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFISystemTable<'a> {
    pub header: EFITableHeader,
    pub firmware_vendor: CChar,
    pub firmware_revision: u32,
    _padding0: [EFIHandle; 3],
    pub con_out: &'a EFISimpleTextOutputProtocol,
    _padding1: [EFIHandle; 3],
    pub boot_services: &'a EFIBootServices,
}

const _: () = {
    use core::mem::offset_of;
    ["con_out"][offset_of!(EFISystemTable, con_out) - 64];
    ["boot_services"][offset_of!(EFISystemTable, boot_services) - 96];
};

#[repr(C)]
#[derive(Debug)]
pub struct EFISimpleTextOutputProtocol {
    _padding0: [EFIHandle; 1],
    output_string: fn(*const EFISimpleTextOutputProtocol, *const u16) -> EFIStatus,
    _padding1: [EFIHandle; 4],
    clear_screen: fn(*const EFISimpleTextOutputProtocol) -> EFIStatus,
}

const _: () = {
    use core::mem::offset_of;
    ["output_string"][offset_of!(EFISimpleTextOutputProtocol, output_string) - 8];
    ["clear_screen"][offset_of!(EFISimpleTextOutputProtocol, clear_screen) - 48];
};

impl EFISimpleTextOutputProtocol {
    pub fn output_string(&self, c: *const u16) {
        (self.output_string)(self, c);
    }

    pub fn write_char(&self, c: char) {
        let buf = [c as u16, 0];
        self.output_string(buf.as_ptr());
    }

    pub fn clear_screen(&self) {
        (self.clear_screen)(self);
    }
}

pub struct EFISimpleTextWriter<'a> {
    protocol: &'a EFISimpleTextOutputProtocol,
}

impl<'a> EFISimpleTextWriter<'a> {
    pub fn new(protocol: &'a EFISimpleTextOutputProtocol) -> Self {
        Self { protocol }
    }
}

impl<'a> fmt::Write for EFISimpleTextWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.chars().for_each(|c| {
            if c.eq(&'\n') {
                self.protocol.write_char('\r');
            }
            self.protocol.write_char(c);
        });
        Ok(())
    }
}

pub static WRITER: GlobalWriter = GlobalWriter::new();

pub struct GlobalWriter {
    pub writer: RefCell<Option<EFISimpleTextWriter<'static>>>,
}

unsafe impl Sync for GlobalWriter {}

impl GlobalWriter {
    const fn new() -> Self {
        Self {
            writer: RefCell::new(None),
        }
    }

    fn init(&self, writer: EFISimpleTextWriter<'static>) {
        *self.writer.borrow_mut() = Some(writer);
    }
}

pub fn init_text_writer(system_table: &'static EFISystemTable) {
    let writer = EFISimpleTextWriter::new(system_table.con_out);
    WRITER.init(writer);
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIBootServices {
    pub header: EFITableHeader,
    _padding0: [EFIHandle; 4],
    get_memory_map: fn(*mut usize, *mut u8, *mut usize, *mut usize, *mut u32) -> EFIStatus,
    _padding1: [EFIHandle; 32],
    locate_protocol: fn(*const EFIGuid, *const u8, *mut *mut u8) -> EFIStatus,
}

const _: () = {
    use core::mem::offset_of;
    ["get_memory_map"][offset_of!(EFIBootServices, get_memory_map) - 56];
    ["locate_protocol"][offset_of!(EFIBootServices, locate_protocol) - 320];
};

impl EFIBootServices {
    pub fn get_memory_map(&self) -> MemoryMap {
        let mut memory_map = MemoryMap::default();
        (self.get_memory_map)(
            &mut memory_map.size,
            memory_map.buf.as_mut_ptr(),
            &mut memory_map.map_key,
            &mut memory_map.descriptor_size,
            &mut memory_map.version,
        );
        memory_map
    }

    pub fn locate_protocol<'a, T: Guid>(&self) -> &'a T {
        let mut p = ptr::null_mut();
        (self.locate_protocol)(&T::guid(), ptr::null(), &mut p);
        unsafe { &*(p as *mut T) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIGraphicsOutputProtocol<'a> {
    _padding0: [EFIHandle; 3],
    pub mode: &'a EFIGraphicsOutputProtocolMode,
}

impl<'a> Guid for EFIGraphicsOutputProtocol<'a> {
    fn guid() -> EFIGuid {
        EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIGraphicsOutputProtocolMode {
    pub max_mode: u32,
    pub mode: u32,
    info: EFIHandle,
    pub size: usize,
    pub frame_buffer_base: u64,
    pub frame_buffer_size: usize,
}
