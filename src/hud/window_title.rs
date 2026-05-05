use bevy::prelude::*;

pub fn update_window_title(
    mut window_query: Query<&mut Window>,
    mut resize_events: MessageReader<bevy::window::WindowResized>,
) {
    for ev in resize_events.read() {
        if let Ok(mut window) = window_query.get_mut(ev.window) {
            window.title = format!("CW: Window Resolution {:?}", (ev.width, ev.height));
        }
    }
}
