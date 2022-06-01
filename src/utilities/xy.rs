use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}
