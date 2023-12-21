use super::{side::Axis, Quad, Side, VOXEL_SIZE};

pub struct Face<'a> {
    // The `side` field represents the side of a voxel.
    pub side: Side,
    // The `quad` field is a reference to a `Quad`.
    pub quad: &'a Quad,
}

// Implement methods for `Face`.
impl<'a> Face<'a> {
    // Public method `colors` returns a vector of RGBA color values.
    pub fn colors(&self) -> Vec<[f32; 4]> {
        vec![self.quad.color.as_linear_rgba(); 4]
    }

    // Public method `indices` returns an array of indices.
    pub fn indices(&self, start: u32) -> [u32; 6] {
        [start, start + 2, start + 1, start + 1, start + 2, start + 3]
    }

    /// Public method `positions` returns an array of positions.
    /// Each position is a 3D point represented as an array of three `f32` values.
    /// The positions are calculated based on the `side` and `quad` fields of the `Face` struct.
    /// The `side` field determines the orientation of the face, and the `quad` field provides the position of the voxel.
    /// The method first determines the relative positions of the vertices based on the `side` field.
    /// Then it calculates the absolute positions of the vertices based on the `voxel` field of the `quad` field.
    /// Finally, it adds the relative positions to the absolute position of the voxel to get the absolute positions of the vertices.
    pub fn positions(&self) -> [[f32; 3]; 4] {
        // Determine the relative positions of the vertices based on the `side` field.
        let positions = match (&self.side.axis, &self.side.positive) {
            (Axis::X, false) => [
                [-0.5, -0.5, 0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, 0.5],
                [-0.5, 0.5, -0.5],
            ],
            (Axis::X, true) => [
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, -0.5],
                [0.5, 0.5, 0.5],
            ],
            (Axis::Y, false) => [
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
            ],
            (Axis::Y, true) => [
                [-0.5, 0.5, 0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, 0.5],
                [0.5, 0.5, -0.5],
            ],
            (Axis::Z, false) => [
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
            ],
            (Axis::Z, true) => [
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
            ],
        };

        // Calculate the absolute position of the voxel based on the `voxel` field of the `quad` field.
        let (x, y, z) = (
            (self.quad.voxel[0] - 1) as f32,
            (self.quad.voxel[1] - 1) as f32,
            (self.quad.voxel[2] - 1) as f32,
        );

        // Scale the voxel grid to the desired size.
        let voxel_size = VOXEL_SIZE;

        // Add the relative positions to the absolute position of the voxel to get the absolute positions of the vertices.
        [
            [
                x * voxel_size + positions[0][0] * voxel_size,
                y * voxel_size + positions[0][1] * voxel_size,
                z * voxel_size + positions[0][2] * voxel_size,
            ],
            [
                x * voxel_size + positions[1][0] * voxel_size,
                y * voxel_size + positions[1][1] * voxel_size,
                z * voxel_size + positions[1][2] * voxel_size,
            ],
            [
                x * voxel_size + positions[2][0] * voxel_size,
                y * voxel_size + positions[2][1] * voxel_size,
                z * voxel_size + positions[2][2] * voxel_size,
            ],
            [
                x * voxel_size + positions[3][0] * voxel_size,
                y * voxel_size + positions[3][1] * voxel_size,
                z * voxel_size + positions[3][2] * voxel_size,
            ],
        ]
    }

    // Public method `normals` returns an array of normals.
    pub fn normals(&self) -> [[f32; 3]; 4] {
        self.side.normals()
    }

    // Public method `uvs` returns an array of UV coordinates.
    // The UV coordinates are flipped based on the `flip_u` and `flip_v` parameters.
    pub fn uvs(&self, flip_u: bool, flip_v: bool) -> [[f32; 2]; 4] {
        match (flip_u, flip_v) {
            (true, true) => [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            (true, false) => [[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            (false, true) => [[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
            (false, false) => [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        }
    }

    // Public method `voxel` returns the voxel of the `Quad`.
    pub fn voxel(&self) -> [usize; 3] {
        self.quad.voxel
    }
}
