mod detail {
    extern "C" {
        pub fn log(ptr: *const u8, count: usize);
        pub fn output(val: u8);
    }
}

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
