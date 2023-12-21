#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Position {
    pub x: i32, // The x-coordinate of the position
    pub y: i32, // The y-coordinate of the position
    pub z: i32, // The z-coordinate of the position
}

impl Position {
    // Method to calculate the Euclidean distance between this position and another
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = other.x - self.x; // Calculate the difference in x-coordinates
        let dy = other.y - self.y; // Calculate the difference in y-coordinates
        let dz = other.z - self.z; // Calculate the difference in z-coordinates

        // Square the differences, sum them, cast to f64, and take the square root to get the distance
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}
