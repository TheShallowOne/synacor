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
pub extern "C" fn load_image(ptr: *mut u8, len: usize, debug: bool) -> bool {
    if len % 2 != 0 {
        ::js::log("Not a valid image");
        return false;
    }

    let data = to_slice(ptr, len);
    let m = Machine::new_u8(data, debug);

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

enum ExecuteResult {
    Error = -1,
    Ok = 0,
    Halt = 1,
    NeedInput = 2,
    NoImage = 3,
}

#[no_mangle]
pub extern "C" fn execute_step() -> i32 {
    let mut s = unsafe { STATE.0.as_mut() };

    let res = match s {
        Some(ref mut m) => match m.execute() {
            Ok(_) => ExecuteResult::Ok,
            Err(ExecuteError::Halt) => ExecuteResult::Halt,
            Err(ExecuteError::Error(s)) => {
                ::js::log(&s);
                ExecuteResult::Error
            }
            Err(ExecuteError::NeedInput) => ExecuteResult::NeedInput,
        },
        None => ExecuteResult::NoImage,
    };
    res as i32
}

#[no_mangle]
pub extern "C" fn add_input(val: u8) -> bool {
    let mut s = unsafe { STATE.0.as_mut() };

    if let Some(ref mut m) = s {
        m.push_input(val);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn do_output() {
    let mut s = unsafe { STATE.0.as_mut() };

    if let Some(ref mut m) = s {
        while let Some(c) = m.pop_output() {
            ::js::output(c);
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
