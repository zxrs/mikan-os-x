#![no_std]
#![no_main]

mod panic;
mod x86;

#[unsafe(no_mangle)]
fn kernel_main() -> ! {
    loop {
        x86::halt();
    }
}
