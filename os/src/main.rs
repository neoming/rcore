#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod config;
mod loader;
mod sync;
mod syscall;
mod task;
mod trap;
mod timer;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as usize as *mut u8,
            ebss as usize - sbss as usize
        ).fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    info!("[kernel] Hello, rCore!");
    trap::init();
    info!("[kernel] trap init done");
    loader::load_apps();
    info!("[kernel] load apps done");
    trap::enable_timer_interrupt();
    info!("[kernel] enable timer interrupt done");
    timer::set_next_trigger();
    info!("[kernel] set next trigger done");
    task::run_first_task();
    panic!("Unreachable in rust_main!")
}
