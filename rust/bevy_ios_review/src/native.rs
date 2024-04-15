#[cfg(target_os = "ios")]
extern "C" {
    fn ios_request_review(view: *mut std::ffi::c_void);
}

#[cfg(target_os = "ios")]
pub fn request_review(view: *mut std::ffi::c_void) {
    let result = std::panic::catch_unwind(|| unsafe {
        ios_request_review(view);
    });

    if let Err(e) = result {
        bevy::log::error!("ffi error: {e:?}")
    }
}

#[cfg(not(target_os = "ios"))]
pub fn request_review(_view: *mut std::ffi::c_void) {
    bevy::log::info!("request_review called");
}
