extern crate notify_rust;
use notify_rust::Notification;
use notify_rust as notify;
mod ffi;

#[no_mangle]
pub extern fn get_capabilities() -> *const i8
{
    ffi::string_out(notify::get_capabilities().connect(","))
}

#[no_mangle]
pub extern fn notify_show(body_buf:*const i8)
{
    let body = ffi::string_int(body_buf);
    Notification::new().body(&body).show();
}
