use std::{ffi::CString, os::raw::c_char, ptr};

use crate::common;
use crate::dataframe::{DataFrameResult, DataFrameResultCode};
use polars::prelude::*;

#[no_mangle]
pub unsafe extern "C" fn lcp_read_csv(path: *const c_char) -> DataFrameResult {
    let path_str = common::cstr_to_str(path);
    let reader = CsvReader::from_path(path_str);

    return match reader {
        Ok(r) => match r.finish() {
            Ok(df) => DataFrameResult::new(
                Box::into_raw(Box::new(df)),
                DataFrameResultCode::Success,
                CString::new("").unwrap().into_raw(),
            ),
            Err(e) => DataFrameResult::new(
                ptr::null_mut(),
                DataFrameResultCode::CSVParseError,
                CString::new(e.to_string()).unwrap().into_raw(),
            ),
        },
        Err(e) => DataFrameResult::new(
            ptr::null_mut(),
            DataFrameResultCode::CSVReadError,
            CString::new(e.to_string()).unwrap().into_raw(),
        ),
    };
}
