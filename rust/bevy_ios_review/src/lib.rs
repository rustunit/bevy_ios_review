mod native;

use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Event, Copy, Clone, Debug)]
pub struct IosRequestReview;

pub struct IosRequestReviewPlugin;

impl Plugin for IosRequestReviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IosRequestReview>().add_systems(
            Update,
            process_events.run_if(on_event::<IosRequestReview>()),
        );
    }
}

#[allow(deprecated)]
fn process_events(
    mut events: EventReader<IosRequestReview>,
    windows: NonSend<bevy::winit::WinitWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    while let Some(_) = events.read().next() {
        let entity = window_query.single();
        let raw_window = windows.get_window(entity).unwrap();
        match winit::raw_window_handle::HasRawWindowHandle::raw_window_handle(&raw_window) {
            Ok(winit::raw_window_handle::RawWindowHandle::UiKit(ios_handle)) => {
                let ui_window: *mut std::ffi::c_void = ios_handle.ui_view.as_ptr();
                native::request_review(ui_window);
            }
            _ => warn!("Unsupported window."),
        }
    }
}
