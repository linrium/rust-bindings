use std::os::raw::c_char;
use serde::{Deserialize, Serialize};
use std::ffi::CString;

#[derive(Debug, Deserialize, Serialize)]
struct Ip {
    origin: String,
}

#[repr(C)]
struct CIp {
    origin: *const c_char,
}

async fn fetch_data() -> Result<Ip, reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<Ip>()
        .await?;

    Ok(resp)
}

#[no_mangle]
pub extern "C" fn hello() -> *mut CIp {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let ip = rt.block_on(fetch_data()).unwrap();
    let c_string = CString::new(ip.origin).unwrap();
    let c_ip = CIp {
        origin: c_string.into_raw(),
    };

    Box::into_raw(Box::new(c_ip))
}

#[no_mangle]
pub extern "C" fn free_struct(ptr: *mut CIp) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw((*ptr).origin as *mut c_char);
        drop(Box::from_raw(ptr));
    }
}
