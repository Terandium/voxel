use super::{side::Axis, Quad, Side, VOXEL_SIZE};

pub struct Face<'a> {
    pub side: Side,
    pub quad: &'a Quad,
}

impl<'a> Face<'a> {
    pub fn colors(&self) -> Vec<[f32; 4]> {
        vec![self.quad.color.as_linear_rgba(); 4]
    }

    pub fn indices(&self, start: u32) -> [u32; 6] {
        [start, start + 2, start + 1, start + 1, start + 2, start + 3]
    }

    pub fn positions(&self) -> [[f32; 3]; 4] {
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

        let (x, y, z) = (
            (self.quad.voxel[0] - 1) as f32,
            (self.quad.voxel[1] - 1) as f32,
            (self.quad.voxel[2] - 1) as f32,
        );

        let voxel_size = VOXEL_SIZE;

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

    pub fn normals(&self) -> [[f32; 3]; 4] {
        self.side.normals()
    }

    pub fn uvs(&self, flip_u: bool, flip_v: bool) -> [[f32; 2]; 4] {
        match (flip_u, flip_v) {
            (true, true) => [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            (true, false) => [[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            (false, true) => [[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
            (false, false) => [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        }
    }

    pub fn voxel(&self) -> [usize; 3] {
        self.quad.voxel
    }
}
