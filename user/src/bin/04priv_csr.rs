#![no_std]
#![no_main]
#![feature(llvm_asm)]

use riscv::register::sstatus::{self, SPP};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> isize {
    println!("Hello, world!");
    let mut sstatus = sstatus::read();
    sstatus.set_spp(SPP::User);
    0
}
