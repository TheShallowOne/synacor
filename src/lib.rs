#![no_std]
#![feature(alloc)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(global_allocator)]
#![feature(lang_items)]

#[macro_use]
extern crate alloc;
extern crate rlibc;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(_args: ::core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    use core::intrinsics;
    unsafe {
        intrinsics::abort();
    }
}

mod helper;
mod instruction;
mod js;
mod machine;
mod operation;
pub mod wasm;
