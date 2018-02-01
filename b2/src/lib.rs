extern crate a;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;

pub use a::{world_pointer_t};

#[no_mangle]
pub extern "C" fn make_world() -> *mut world_pointer_t {
    a::make_world()
}

#[no_mangle]
pub extern "C" fn print_world(ptr: *mut world_pointer_t) {
    a::print_world(ptr)
}

#[no_mangle]
pub extern "C" fn goodbye_world() -> *const c_char {
    let cstr = CString::new("Goodbye, world!").unwrap();
    let ptr = cstr.as_ptr();
    mem::forget(cstr);
    ptr
}
