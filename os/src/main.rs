#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

use log::{debug, error, info, trace, warn};

#[macro_use]
mod logging;

mod config;
mod console;
mod cpu;
mod drivers;
mod fs;
mod lang_items;
mod loader;
mod mm;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clean_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    unsafe {
        cpu::set_cpu_id(0);
    }
    clean_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    error!("This is a error msg.");
    warn!("This is a warn msg.");
    info!("This is a info msg.");
    debug!("This is a debug mesg.");
    trace!("This is a trace msg.");
    println!("Powered by Rust with ♥");

    mm::init();
    println!("[kernel] back to world!");
    mm::remap_test();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    fs::list_apps();
    task::add_initproc();
    println!("after initproc!");
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}
