extern crate a;

use std::ffi::CString;
use std::os::raw::c_char;

pub use a::{world_pointer_t, make_world, print_world};

#[no_mangle]
pub extern "C" fn goodbye_world() -> *const c_char {
    CString::new("Goodbye, world!").unwrap().as_ptr()
}
