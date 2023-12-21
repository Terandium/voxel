pub const EMPTY: Visibility = Visibility::Empty;
pub const OPAQUE: Visibility = Visibility::Opaque;
pub const TRANSPARENT: Visibility = Visibility::Transparent;

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    pub fn as_linear_rgba(&self) -> [f32; 4] {
        [
            self.red as f32 / 255.0,
            self.green as f32 / 255.0,
            self.blue as f32 / 255.0,
            self.alpha as f32 / 255.0,
        ]
    }
}

impl From<Color> for bevy::render::color::Color {
    fn from(color: Color) -> Self {
        Self::rgba(
            color.red as f32 / 255.0,
            color.green as f32 / 255.0,
            color.blue as f32 / 255.0,
            color.alpha as f32 / 255.0,
        )
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Visibility {
    Empty,
    Opaque,
    Transparent,
}

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
