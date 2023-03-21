use std::{
    ffi::{c_int, CString},
    ptr,
};

use polars::{lazy::dsl::Expr, prelude::*};

use crate::common::to_raw;
use crate::dataframe::{DataFrameResult, DataFrameResultCode};

#[no_mangle]
pub unsafe extern "C" fn lcp_collect(lf: *mut LazyFrame) -> DataFrameResult {
    let res = (&*lf).clone().collect();
    match res {
        Ok(df) => DataFrameResult::new(
            to_raw(df),
            DataFrameResultCode::Success,
            CString::new("").unwrap().into_raw(),
        ),
        Err(e) => DataFrameResult::new(
            ptr::null_mut(),
            DataFrameResultCode::CollectError,
            CString::new(e.to_string()).unwrap().into_raw(),
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn lcp_filter(lf: *mut LazyFrame, predicate: *mut Expr) -> *mut LazyFrame {
    to_raw((&*lf).clone().filter((&*predicate).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn lcp_select(
    lf: *mut LazyFrame,
    exprs: *const *mut Expr,
    len: i32,
) -> *mut LazyFrame {
    let len = c_int::try_into(len).unwrap();
    let exprs_slice = unsafe { std::slice::from_raw_parts(exprs, len) };
    let mut exprs_vec = Vec::new();
    for &expr_ptr in exprs_slice {
        let expr = (*expr_ptr).clone();
        exprs_vec.push(expr);
    }

    to_raw((&*lf).clone().select(&exprs_vec))
}
