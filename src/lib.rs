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

use alloc::String;

mod helper;
mod instruction;
mod js;
mod machine;
mod operation;
pub mod wasm;

pub struct Machine {
    detail: machine::MachineDetail,
}

impl Machine {
    pub const fn new() -> Machine {
        Machine {
            detail: machine::MachineDetail::new(),
        }
    }

    pub fn load(&mut self, input: &[u16]) -> Result<(), String> {
        self.detail.load(input)
    }

    pub fn load_u8(&mut self, input: &[u8]) -> Result<(), String> {
        // little endian
        let mut data = vec![0; input.len() / 2];

        for (i, byte) in input.iter().enumerate() {
            let idx = i / 2;
            let shift = (i % 2) * 8;

            let byte = (*byte as u16) << shift;
            data[idx] |= byte;
        }

        self.detail.load(&data)
    }

    pub fn execute(&mut self) -> Result<bool, String> {
        self.detail.execute()
    }
}
