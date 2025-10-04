use std::{ffi::{CStr, CString, c_char}, sync::OnceLock};

use crate::SDKApp;

static APPID: OnceLock<SDKApp> = OnceLock::new();

fn set_appid(s: *const c_char) -> Option<()> {
  let st: CString = unsafe { CStr::from_ptr(s) }.to_owned();

  APPID.set(SDKApp::create_from_cstring(st)).ok()
}

#[unsafe(no_mangle)]
/// Initializes the sdk with the app_id provided
/// 
/// We get a reference to this and then use it when required
/// This function can only be called **once**
/// 
/// You are supposed to provide and manage **char** yourself
/// We create a copy of this **char** and use it
/// 
/// **You are supposed to provide a non null char**
/// 
/// # Safety
/// We copy an instance of `app_id` and set it in our static
/// storage. You can safely deallocate the app_id char
/// 
/// You must call this first
pub extern "C" fn init_sdk(app_id: *const c_char) -> u8 {
  match set_appid(app_id) {
    Some(_) => 0,
    None => 1
  }
}