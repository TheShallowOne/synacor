use alloc::Vec;
use core::mem;
use core::slice;

static mut MACHINE: ::Machine = ::Machine::new();

fn to_slice_u16<'a>(ptr: *const u8, len: usize) -> &'a [u16] {
    unsafe { slice::from_raw_parts(ptr as *const u16, len) }
}

#[no_mangle]
pub extern "C" fn load_image(ptr: *mut u8, len: usize) -> bool {
    if len % 2 != 0 {
        ::js::log("Not a valid image");
        return false;
    }

    let res = unsafe { MACHINE.load(to_slice_u16(ptr, len)) };
    match res {
        Ok(_) => true,
        Err(s) => {
            ::js::log(&s);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut u8;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn test_imports() {
    use js::*;
    log("Log message");
    output(123);
}

/*
pub fn log(message: &str) {
    let bytes = message.as_bytes();
    let count = bytes.len();

    unsafe {
        let ptr = bytes.as_ptr();
        detail::log(ptr, count);
    }
}

pub fn output(val: u8) {
    unsafe {
        detail::output(val);
    }
}
*/