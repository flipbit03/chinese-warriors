use bevy::{
    input::Input,
    prelude::{info, KeyCode, OrthographicProjection, Query, Res, With},
    render::camera::Camera2d,
};

pub fn input_camera_scale(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    let mut camera = camera_query.single_mut();

    let mut scale = camera.scale;

    if keyboard_input.pressed(KeyCode::Q) {
        scale = (scale * 100.0).round();
        scale += 3.0;
        scale /= 100.0;
        camera.scale = scale;
        info!("camera.scale = {}", &camera.scale);
    }

    if keyboard_input.pressed(KeyCode::E) {
        scale = (scale * 100.0).round();
        scale -= 3.0;
        scale /= 100.0;

        info!("camera.scale = {}", &camera.scale);
    }

    camera.scale = scale.max(0.5).min(2.0);
}
