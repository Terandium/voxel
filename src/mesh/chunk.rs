use bevy::{
    ecs::{component::Component, entity::Entity, system::Resource},
    utils::HashMap,
};
use libnoise::prelude::*;

use crate::{
    mesh::Quad,
    util::{Color, Visibility, EMPTY, OPAQUE, TRANSPARENT},
    Position,
};

use super::{QuadGroups, Voxel};

pub const CHUNK_SIZE: f32 = 24.0;

#[derive(Resource, Default)]
pub struct LoadedChunks(pub HashMap<Position, Entity>);

#[derive(Component, Eq, PartialEq)]
pub struct Chunk {
    pub position: Position,
}

impl Chunk {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

#[derive(Clone, Debug)]
pub struct ChunkMesh {
    pub voxels: Vec<Voxel>,
    pub position: Position,
}

impl ChunkMesh {
    pub const X: usize = CHUNK_SIZE as usize + 2;
    pub const Y: usize = CHUNK_SIZE as usize + 2;
    pub const Z: usize = CHUNK_SIZE as usize + 2;

    pub fn new(position: Position) -> Self {
        Self {
            voxels: Vec::with_capacity(Self::size()),
            position,
        }
    }

    pub fn size() -> usize {
        Self::X * Self::Y * Self::Z
    }

    pub fn linearize(x: usize, y: usize, z: usize) -> usize {
        x + Self::X * (y + Self::Y * z)
    }

    pub fn delinearize(index: usize) -> (usize, usize, usize) {
        let (x, index) = (index % Self::X, index / Self::X);
        let (y, z) = (index % Self::X, index / Self::Y);
        (x, y, z)
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Voxel {
        self.voxels[Self::linearize(x, y, z)]
    }

    pub fn populate(&mut self, seed: u64) {
        let generator = Source::simplex(seed)
            .fbm(4, 0.01, 2.0, 0.8)
            .blend(
                Source::worley(seed + 1).scale([0.05, 0.05]),
                Source::worley(seed + 2).scale([0.02, 0.02]),
            )
            .lambda(|f| ((f * 2.0).sin() * 0.3 + f * 0.7) * 20.0);

        for i in 0..ChunkMesh::size() {
            let (local_x, local_y, local_z) = ChunkMesh::delinearize(i);

            let (x, y, z) = (
                local_x as i32 + &self.position.x * CHUNK_SIZE as i32,
                local_y as i32 + &self.position.y * CHUNK_SIZE as i32,
                local_z as i32 + &self.position.z * CHUNK_SIZE as i32,
            );

            let val = generator.sample([x as f64, z as f64]);

            if y <= 1 {
                self.voxels.push(Voxel::Opaque(Color::new(0, 0, 255)));
            } else {
                if y <= val as i32 || y <= 1 {
                    if y <= 3 {
                        self.voxels.push(Voxel::Opaque(Color::new(242, 231, 122)));
                    } else if y <= 20 {
                        self.voxels.push(Voxel::Opaque(Color::new(146, 142, 133)));
                    } else {
                        self.voxels.push(Voxel::Opaque(Color::new(255, 250, 250)));
                    }
                } else {
                    self.voxels.push(Voxel::Empty);
                }
            }
        }
    }

    pub fn generate_mesh(&self) -> QuadGroups {
        let mut buffer = QuadGroups::default();

        for i in 0..ChunkMesh::size() {
            let (x, y, z) = ChunkMesh::delinearize(i);
            if (x > 0 && x < ChunkMesh::X - 1)
                && (y > 0 && y < ChunkMesh::Y - 1)
                && (z > 0 && z < ChunkMesh::Z - 1)
            {
                let voxel = self.get(x, y, z);

                match voxel.visibility() {
                    Visibility::Empty => continue,
                    visibility => {
                        let neighbors = [
                            self.get(x - 1, y, z),
                            self.get(x + 1, y, z),
                            self.get(x, y - 1, z),
                            self.get(x, y + 1, z),
                            self.get(x, y, z - 1),
                            self.get(x, y, z + 1),
                        ];

                        for (i, neighbor) in neighbors.into_iter().enumerate() {
                            let other = neighbor.visibility();

                            let generate = match (visibility, other) {
                                (OPAQUE, EMPTY) | (OPAQUE, TRANSPARENT) | (TRANSPARENT, EMPTY) => {
                                    true
                                }

                                (TRANSPARENT, TRANSPARENT) => voxel != neighbor,

                                (_, _) => false,
                            };

                            match voxel {
                                Voxel::Opaque(color) | Voxel::Transparent(color) => {
                                    if generate {
                                        buffer.groups[i].push(Quad {
                                            voxel: [x, y, z],
                                            width: 1,
                                            height: 1,
                                            color,
                                        });
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        buffer
    }
}
