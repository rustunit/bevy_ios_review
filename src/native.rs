#[cfg(target_os = "ios")]
pub fn request_review(view: *mut std::ffi::c_void) {
    use objc2::{ClassType, msg_send};
    use objc2_ui_kit::UIView;

    let Some(view) = std::ptr::NonNull::new(view.cast::<UIView>()) else {
        bevy_log::warn!("no view to request review from");
        return;
    };

    // Caller guarantees this is the live `UIView` backing the winit window, and Bevy's
    // `NonSendMarker` keeps us on the main thread that UIKit requires.
    let view = unsafe { view.as_ref() };

    let Some(scene) = view.window().and_then(|window| window.windowScene()) else {
        bevy_log::warn!("view is not attached to a window scene");
        return;
    };

    // `objc2-store-kit` only generates the scene-less variant, so send the selector by hand.
    #[allow(deprecated)]
    let class = objc2_store_kit::SKStoreReviewController::class();

    unsafe {
        let _: () = msg_send![class, requestReviewInScene: &*scene];
    }
}

#[cfg(not(target_os = "ios"))]
pub fn request_review(_view: *mut std::ffi::c_void) {
    bevy_log::info!("request_review called");
}
