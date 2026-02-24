#![no_std]
#![no_main]

#[macro_use]
mod macros;
mod memory_map;
mod panic;
mod uefi;
mod x86;

use core::{fmt::Write, slice};
use memory_map::MemoryMapVisitor;
use uefi::{EFIGraphicsOutputProtocol, EFIHandle, EFISystemTable, init_text_writer};

#[unsafe(no_mangle)]
fn efi_main(_: EFIHandle, system_table: &'static EFISystemTable) -> ! {
    system_table.con_out.clear_screen();

    init_text_writer(system_table);

    let gop = system_table
        .boot_services
        .locate_protocol::<EFIGraphicsOutputProtocol>();
    let base = gop.mode.frame_buffer_base as *mut u8;
    let size = gop.mode.frame_buffer_size;
    let frame_buffer = unsafe { slice::from_raw_parts_mut(base, size) };
    frame_buffer.fill(255);

    println!("{}", system_table.firmware_vendor);

    let memory_map = system_table.boot_services.get_memory_map();

    let visitor = MemoryMapVisitor::new(&memory_map);
    visitor.for_each(|d| {
        println!("{:?}, {:x?}", d.typ, d.physical_address);
    });

    loop {
        x86::halt();
    }
}
