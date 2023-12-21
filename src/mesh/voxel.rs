use crate::util::{Color, Visibility};

/// The size of a voxel
pub const VOXEL_SIZE: f32 = 1.0;

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum Voxel {
    #[default]
    Empty,
    Opaque(Color),
    Transparent(Color),
}

impl Voxel {
    pub fn visibility(&self) -> Visibility {
        match self {
            Self::Empty => Visibility::Empty,
            Self::Opaque(_) => Visibility::Opaque,
            Self::Transparent(_) => Visibility::Transparent,
        }
    }
}
