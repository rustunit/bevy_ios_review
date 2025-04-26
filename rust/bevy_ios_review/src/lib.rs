mod native;

use bevy::{prelude::*, window::PrimaryWindow};
use winit::raw_window_handle::HasWindowHandle;

#[derive(Event, Copy, Clone, Debug)]
pub struct IosRequestReview;

pub struct IosRequestReviewPlugin;

impl Plugin for IosRequestReviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IosRequestReview>()
            .add_systems(Update, process_events.run_if(on_event::<IosRequestReview>));
    }
}

fn process_events(
    mut events: EventReader<IosRequestReview>,
    windows: NonSend<bevy::winit::WinitWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    while let Some(_) = events.read().next() {
        let Ok(entity) = window_query.single() else {
            error!("primary window not found");
            continue;
        };
        let raw_window = windows.get_window(entity).expect("invalid window handle");
        if let Ok(handle) = raw_window.window_handle() {
            if let winit::raw_window_handle::RawWindowHandle::UiKit(handle) = handle.as_raw() {
                let ui_window: *mut std::ffi::c_void = handle.ui_view.as_ptr();
                native::request_review(ui_window);
                continue;
            }
        }
        warn!("Unsupported window.")
    }
}
