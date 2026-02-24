pub const MEMORY_MAP_SIZE: usize = 4096 * 4;

#[derive(Debug)]
pub struct MemoryMap {
    pub size: usize,
    pub buf: [u8; MEMORY_MAP_SIZE],
    pub map_key: usize,
    pub descriptor_size: usize,
    pub version: u32,
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap {
            size: MEMORY_MAP_SIZE,
            buf: [0; _],
            map_key: 0,
            descriptor_size: 0,
            version: 0,
        }
    }
}

#[derive(Debug)]
pub struct MemoryMapVisitor<'a> {
    memory_map: &'a MemoryMap,
    offset: usize,
}

impl<'a> MemoryMapVisitor<'a> {
    pub fn new(memory_map: &'a MemoryMap) -> Self {
        Self {
            memory_map,
            offset: 0,
        }
    }
}

impl<'a> Iterator for MemoryMapVisitor<'a> {
    type Item = &'a EFIMemoryDescriptor;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.memory_map.size {
            return None;
        }
        let descritptor = unsafe {
            &*(self.memory_map.buf.as_ptr().add(self.offset) as *const EFIMemoryDescriptor)
        };
        self.offset += self.memory_map.descriptor_size;
        Some(descritptor)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIMemoryDescriptor {
    pub typ: EFIMemoryType,
    pub physical_address: u64,
    pub virtusl_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[allow(unused)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EFIMemoryType {
    ReservedMemoryType = 0,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemoryType,
    MaxMemoryType,
}
