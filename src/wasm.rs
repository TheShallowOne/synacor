use alloc::Vec;
use core::mem;
use core::slice;
use operation::ExecuteError;
use machine::Machine;

struct State(Option<Machine>);

impl State {
    const fn new() -> State {
        State(None)
    }
}
static mut STATE: State = State::new();


fn to_slice<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    unsafe { slice::from_raw_parts(ptr, len) }
}

#[no_mangle]
pub extern "C" fn load_image(ptr: *mut u8, len: usize) -> bool {
    if len % 2 != 0 {
        ::js::log("Not a valid image");
        return false;
    }

    let data = to_slice(ptr, len);
    let m = Machine::new_u8(data);

    match m {
        Ok(m) => {
            unsafe {
                STATE.0 = Some(m);
            }
            true
        }
        Err(s) => {
            ::js::log(&s);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn execute_step() -> bool {
    let mut s = unsafe { STATE.0.as_mut() };

    match s {
        Some(ref mut m) => match m.execute() {
            Ok(_) => true,
            Err(ExecuteError::Halt) => false,
            Err(ExecuteError::Error(s)) => {
                ::js::log(&s);
                false
            }
            Err(ExecuteError::NeedInput) => unimplemented!(),
        },
        None => false,
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
