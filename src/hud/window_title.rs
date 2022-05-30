use bevy::{
    prelude::{EventReader, ResMut},
    window::{WindowResized, Windows},
};

pub fn update_window_title(
    mut windows: ResMut<Windows>,
    mut resize_events: EventReader<WindowResized>,
) {
    for ev in resize_events.iter() {
        let window = windows.get_primary_mut().unwrap();
        window.set_title(format!("CW: Window Resolution {:?}", (ev.width, ev.height)));
    }
}
