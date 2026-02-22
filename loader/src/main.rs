#![no_std]
#![no_main]

#[macro_use]
mod macros;
mod memory_map;
mod panic;
mod uefi;
mod x86;

use core::fmt::Write;
use uefi::{EFIHandle, EFISystemTable};

use crate::uefi::init_text_writer;

#[unsafe(no_mangle)]
fn efi_main(_: EFIHandle, system_table: &'static EFISystemTable) -> ! {
    system_table.con_out.clear_screen();

    init_text_writer(system_table);

    println!("{}", system_table.firmware_vendor);

    let memory_map = system_table.boot_services.get_memory_map();
    println!("{:?}", memory_map);

    loop {
        x86::halt();
    }
}
