use bevy::{prelude::*, window::WindowResized};

pub fn window_title_resolution(
    mut windows: ResMut<Windows>,
    mut resize_events: EventReader<WindowResized>,
) {
    for ev in resize_events.iter() {
        let window = windows.get_primary_mut().unwrap();
        window.set_title(format!(
            "CW: Window Resolution({:?})",
            (ev.width, ev.height)
        ));
    }
}
