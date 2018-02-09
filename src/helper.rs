use std::ffi::CStr;
use std::os::raw::c_char;

pub fn convert_string(data: *const c_char) -> String {
    unsafe {
        let s = CStr::from_ptr(data);
        s.to_string_lossy().into_owned()
    }
}