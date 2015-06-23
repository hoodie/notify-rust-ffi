use std::ffi::{CString,CStr};
use std::mem;
use std::str::from_utf8;

pub extern fn string_out(string:String) -> *const i8
{
    let s = CString::new(string).unwrap();
    let p = s.as_ptr();
    mem::forget(s);
    p
}

pub fn string_int(c_buf:*const i8) -> String
{
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice: &str = from_utf8(buf).unwrap();
    str_slice.to_string()
}
