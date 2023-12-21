pub const EMPTY: Visibility = Visibility::Empty;
pub const OPAQUE: Visibility = Visibility::Opaque;
pub const TRANSPARENT: Visibility = Visibility::Transparent;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Visibility {
    Empty,
    Opaque,
    Transparent,
}
