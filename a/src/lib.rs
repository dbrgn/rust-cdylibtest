struct OpaqueWorld {
    data: Vec<String>,
}

/// An opaque pointer to an `OpaqueWorld` instance.
/// Used for C type safety.
#[allow(non_camel_case_types)]
pub enum world_pointer_t {}

#[no_mangle]
pub extern "C" fn make_world() -> *mut world_pointer_t {
    // Create instance
    let inst = OpaqueWorld {
        data: vec!["hello".to_string(), "world".to_string()],
    };

    // Return opaque raw pointer to instance
    Box::into_raw(Box::new(inst)) as *mut world_pointer_t
}

#[no_mangle]
pub extern "C" fn print_world(ptr: *mut world_pointer_t) {
    if ptr.is_null() {
        return;
    }
    let inst = unsafe { Box::from_raw(ptr as *mut OpaqueWorld) };
    for s in inst.data {
        println!("{}", s);
    }
}
