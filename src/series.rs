use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use polars::prelude::*;

#[no_mangle]
pub extern "C" fn lcp_new_int_series(
    name: *const c_char,
    data: *const i32,
    len: usize,
) -> *mut Series {
    let name = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let data = unsafe { std::slice::from_raw_parts(data, len).to_vec() };
    Box::into_raw(Box::new(Series::new(&name, &data)))
}

#[no_mangle]
pub unsafe extern "C" fn lcp_series_to_str(s: *mut Series) -> *mut c_char {
    CString::new((&*s).to_string()).unwrap().into_raw()
}
