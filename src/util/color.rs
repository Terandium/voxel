#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct Color {
    pub red: u8,   // The red component of the color
    pub green: u8, // The green component of the color
    pub blue: u8,  // The blue component of the color
    pub alpha: u8, // The alpha (transparency) component of the color
}

impl Color {
    // A constant function to create a new `Color` instance with the specified red, green, and blue components
    // The alpha component is set to 255 (fully opaque) by default
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    // Method to convert the color to a linear RGBA array of f32 values
    // Each component is divided by 255.0 to normalize it to the range [0.0, 1.0]
    pub fn as_linear_rgba(&self) -> [f32; 4] {
        [
            self.red as f32 / 255.0,
            self.green as f32 / 255.0,
            self.blue as f32 / 255.0,
            self.alpha as f32 / 255.0,
        ]
    }
}

// Implement the `From` trait for converting a `Color` to a `bevy::render::color::Color`
impl From<Color> for bevy::render::color::Color {
    // The conversion is done by creating a new `bevy::render::color::Color` with the RGBA components of the `Color`
    // Each component is divided by 255.0 to normalize it to the range [0.0, 1.0]
    fn from(color: Color) -> Self {
        Self::rgba(
            color.red as f32 / 255.0,
            color.green as f32 / 255.0,
            color.blue as f32 / 255.0,
            color.alpha as f32 / 255.0,
        )
    }
}
