pub enum Axis {
    X,
    Y,
    Z,
}

// Public struct `Side` with fields `axis` and `positive`.
pub struct Side {
    // The `axis` field represents the axis of the side.
    pub axis: Axis,
    // The `positive` field indicates whether the side is in the positive direction of the axis.
    pub positive: bool,
}

// Implement methods for `Side`.
impl Side {
    // Public method `new` creates a new `Side` with the given `axis` and `positive` values.
    pub fn new(axis: Axis, positive: bool) -> Self {
        Self { axis, positive }
    }

    // Public method `normal` returns the normal vector of the side as an array of three `f32` values.
    // The normal vector points in the direction of the side.
    pub fn normal(&self) -> [f32; 3] {
        match (&self.axis, &self.positive) {
            (Axis::X, true) => [1.0, 0.0, 0.0],   // X+
            (Axis::X, false) => [-1.0, 0.0, 0.0], // X-
            (Axis::Y, true) => [0.0, 1.0, 0.0],   // Y+
            (Axis::Y, false) => [0.0, -1.0, 0.0], // Y-
            (Axis::Z, true) => [0.0, 0.0, 1.0],   // Z+
            (Axis::Z, false) => [0.0, 0.0, -1.0], // Z-
        }
    }

    // Public method `normals` returns an array of four normal vectors of the side.
    // This can be useful for operations that require multiple normals of the same side.
    pub fn normals(&self) -> [[f32; 3]; 4] {
        [self.normal(), self.normal(), self.normal(), self.normal()]
    }
}

// Implement the `From<usize>` trait for `Side`.
impl From<usize> for Side {
    // The `from` method converts a `usize` value into a `Side`.
    // The conversion is based on a specific mapping from numbers to sides.
    fn from(value: usize) -> Self {
        match value {
            0 => Self::new(Axis::X, false), // X-
            1 => Self::new(Axis::X, true),  // X+
            2 => Self::new(Axis::Y, false), // Y-
            3 => Self::new(Axis::Y, true),  // Y+
            4 => Self::new(Axis::Z, false), // Z-
            5 => Self::new(Axis::Z, true),  // Z+
            _ => unreachable!(),
        }
    }
}
