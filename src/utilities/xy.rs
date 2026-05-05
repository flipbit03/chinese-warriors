use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize, Component)]
#[repr(C)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}
