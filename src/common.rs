use std::{
    ffi::{c_int, CStr},
    os::raw::c_char,
};

pub unsafe fn cstr_to_str<'a>(s: *const c_char) -> &'a str {
    CStr::from_ptr(s).to_str().unwrap()
}

pub unsafe fn cstr_arr_to_vec(cstr_arr: *const *const c_char, len: c_int) -> Vec<String> {
    let arr = unsafe { std::slice::from_raw_parts(cstr_arr, c_int::try_into(len).unwrap()) };
    let arr: Vec<String> = arr
        .iter()
        .map(|&s| unsafe { CStr::from_ptr(s) })
        .map(|cs| cs.to_str().unwrap().to_string())
        .collect();
    return arr;
}

pub fn to_raw<T>(obj: T) -> *mut T {
    Box::into_raw(Box::new(obj))
}
