use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

mod functions;
mod helper;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn sum_bytes(data: *mut c_char) -> *mut c_char {
    let s = helper::convert_string(data);

    let sum = functions::sum_bytes(s.as_bytes());
    let r = format!("{}", sum);

    let s = CString::new(r).unwrap();
    s.into_raw()
}
