use bevy::{prelude::*, render::camera::Camera2d};

pub fn camera_scale_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    let mut camera = camera_query.single_mut();

    let mut scale = camera.scale;

    if keyboard_input.pressed(KeyCode::Q) {
        scale = (scale * 100.0).round();
        scale += 2.0;
        scale /= 100.0;
        camera.scale = scale;
        println!("camera.scale = {}", &camera.scale);
    }

    if keyboard_input.pressed(KeyCode::E) {
        scale = (scale * 100.0).round();
        scale -= 2.0;
        scale /= 100.0;
        camera.scale = scale;
        println!("camera.scale = {}", &camera.scale);
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_scale_input);
    }
}
