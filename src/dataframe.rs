use crate::common::cstr_arr_to_vec;
use crate::common::to_raw;
use polars::prelude::*;
use std::{
    ffi::{c_int, CString},
    os::raw::c_char,
    ptr,
};

#[repr(C)]
pub enum DataFrameResultCode {
    Success,
    CSVReadError,
    CSVParseError,
    CollectError,
    SelectError,
}

#[repr(C)]
pub struct DataFrameResult {
    dataframe: *mut DataFrame,
    result_code: DataFrameResultCode,
    message: *const c_char,
}

impl DataFrameResult {
    pub fn new(
        dataframe: *mut DataFrame,
        result_code: DataFrameResultCode,
        message: *const c_char,
    ) -> Self {
        DataFrameResult {
            dataframe,
            result_code,
            message,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn lcp_columns(
    df: *mut DataFrame,
    names: *const *const c_char,
    len: c_int,
) -> DataFrameResult {
    let names = cstr_arr_to_vec(names, len);

    match (&*df).select(names) {
        Ok(res) => DataFrameResult {
            dataframe: to_raw(res),
            result_code: DataFrameResultCode::Success,
            message: CString::new("").unwrap().into_raw(),
        },
        Err(e) => DataFrameResult {
            dataframe: ptr::null_mut(),
            result_code: DataFrameResultCode::SelectError,
            message: CString::new(e.to_string()).unwrap().into_raw(),
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn lcp_frame_equal(
    df1: *mut DataFrame,
    df2: *mut DataFrame,
    missing: i32,
) -> i32 {
    if !(missing == 0) {
        return (&*df1).frame_equal_missing(&*df2) as i32;
    }
    return (&*df1).frame_equal(&*df2) as i32;
}

#[no_mangle]
pub unsafe extern "C" fn lcp_head(df: *mut DataFrame, n: i32) -> *mut DataFrame {
    to_raw((&*df).head(Some(n as usize)))
}

#[no_mangle]
pub unsafe extern "C" fn lcp_lazy(df: *mut DataFrame) -> *mut LazyFrame {
    to_raw((&*df).clone().lazy())
}

/// Get string representation of a DataFrame. This can be useful for printing.
#[no_mangle]
pub unsafe extern "C" fn lcp_dataframe_to_str(df: *mut DataFrame) -> *mut c_char {
    CString::new((&*df).to_string()).unwrap().into_raw()
}
