#![no_std]
#![no_main]

mod panic;

#[unsafe(no_mangle)]
fn efi_main() -> ! {
    loop {}
}
