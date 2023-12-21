#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}
