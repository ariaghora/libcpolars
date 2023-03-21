use std::ffi::c_char;

use crate::common::{cstr_to_str, to_raw};
use polars::{lazy::dsl::Expr, prelude::*};

#[no_mangle]
pub extern "C" fn lcp_alloc_expr_arr(len: i32) -> *mut *mut Expr {
    let mut vec = Vec::<*mut Expr>::new();
    vec.resize(len as usize, std::ptr::null_mut());

    let boxed_slice: Box<[*mut Expr]> = vec.into_boxed_slice();
    let ptr = Box::into_raw(boxed_slice) as *mut *mut Expr;

    ptr
}

#[no_mangle]
pub extern "C" fn lcp_set_expr_arr(expr_arr: *mut *mut Expr, expr: *mut Expr, at: i32) {
    let expr_arr = unsafe { std::slice::from_raw_parts_mut(expr_arr, at as usize + 1) };
    expr_arr[at as usize] = expr;
}

#[no_mangle]
pub unsafe extern "C" fn lcp_expr_alias(expr: *mut Expr, name: *const c_char) -> *mut Expr {
    let name = { cstr_to_str(name) };
    to_raw((&*expr).clone().alias(name))
}

#[no_mangle]
pub extern "C" fn lcp_expr_column(name: *const c_char) -> *mut Expr {
    let name = unsafe { cstr_to_str(name) };
    to_raw(Expr::Column(name.into()))
}

#[no_mangle]
pub extern "C" fn lcp_expr_eq(e1: *mut Expr, e2: *mut Expr) -> *mut Expr {
    unsafe { to_raw(Expr::eq((&*e1).clone(), (&*e2).clone())) }
}

#[no_mangle]
pub extern "C" fn lcp_expr_i32(val: i32) -> *mut Expr {
    let val = LiteralValue::Int32(val);
    to_raw(Expr::Literal(val))
}

#[no_mangle]
pub extern "C" fn lcp_expr_str(val: *const c_char) -> *mut Expr {
    let val = unsafe { LiteralValue::Utf8(cstr_to_str(val).to_string()) };
    to_raw(Expr::Literal(val))
}
