#[macro_use]
extern crate errloc_macros;

#[no_mangle]
pub extern "C" fn hello(
        name: *const std::os::raw::c_char,
        name_len: std::os::raw::c_int, 
        out: *mut *const std::os::raw::c_char, 
        out_len: *mut std::os::raw::c_int
) -> *mut std::os::raw::c_char {
    unsafe {
        let res: *mut std::os::raw::c_char = std::panic::catch_unwind(|| {
            // cast c_char to u8
            let uname: *const u8 = name as *const u8;
            let ulen: usize = (name_len) as usize;
            // copy to vec
            let name_vec = std::slice::from_raw_parts(uname, ulen).to_vec();
            // check utf8 and create string
            let name_str = std::string::String::from_utf8(name_vec).expect(errloc!());
            // create result string
            let mut res = name_str;
            res.push_str(", you are rusty");
            res.push('\0');
            res.shrink_to_fit();
            // assign result
            *out = res.as_ptr() as *const std::os::raw::c_char;
            *out_len = (res.len() - 1) as i32;
            // do not deallocate result
            std::mem::forget(res);
            // return null for success
            std::ptr::null_mut::<std::os::raw::c_char>()
        }).unwrap_or_else(|e| {
            // get error message ref
            let err = errloc_macros::msg(&e);
            // convert it to string to add NUL terminator
            let mut err_str = err.to_string();
            err_str.push('\0');
            err_str.shrink_to_fit();
            let res = err_str.as_ptr() as *mut std::os::raw::c_char;
            std::mem::forget(err_str);
            res
        });
        res
    }
}

#[no_mangle]
pub extern "C" fn hello_free(buf: *mut std::os::raw::c_char) -> () {
    unsafe {
        if std::ptr::null_mut() != buf {
            // inspect and re-create the string we returned to client before
            let len = std::ffi::CStr::from_ptr(buf).to_bytes().len();
            let ubuf: *mut u8 = buf as *mut u8;
            std::string::String::from_raw_parts(ubuf, len, len);
            // will work too will less hassle, assuming
            // box and string use the same allocator
            // std::boxed::Box::from_raw(buf);
        }
    }
}
