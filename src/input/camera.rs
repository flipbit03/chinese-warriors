use bevy::prelude::*;

pub fn input_camera_scale(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Projection, With<Camera2d>>,
) {
    let Ok(mut projection) = camera_query.single_mut() else {
        return;
    };
    let Projection::Orthographic(ref mut camera) = *projection else {
        return;
    };

    let mut scale = camera.scale;

    if keyboard_input.pressed(KeyCode::KeyQ) {
        scale = (scale * 100.0).round();
        scale += 3.0;
        scale /= 100.0;
        info!("camera.scale = {}", scale);
    }

    if keyboard_input.pressed(KeyCode::KeyE) {
        scale = (scale * 100.0).round();
        scale -= 3.0;
        scale /= 100.0;
        info!("camera.scale = {}", scale);
    }

    camera.scale = scale.max(0.5).min(4.0);
}
