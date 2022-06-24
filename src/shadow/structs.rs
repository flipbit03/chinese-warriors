use bevy::prelude::Component;

#[derive(Component)]
pub struct CastsShadow {
    pub x_scale: f32,
    pub y_offset: f32,
    pub alpha: f32,
}

impl Default for CastsShadow {
    fn default() -> Self {
        Self {
            x_scale: 1.0,
            y_offset: 0.0,
            alpha: 0.7,
        }
    }
}
