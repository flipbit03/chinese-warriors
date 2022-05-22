use bevy::math::Rect;

pub trait WidthHeight {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
}

impl WidthHeight for Rect<f32> {
    fn width(&self) -> f32 {
        self.right - self.left
    }

    fn height(&self) -> f32 {
        self.top - self.bottom
    }
}
