use std::sync::{Arc, Mutex};

use bevy::{
    ecs::{component::Component, entity::Entity, system::Resource},
    utils::HashMap,
};
use noise::{NoiseFn, Perlin};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    mesh::Quad,
    util::{Color, Visibility, EMPTY, OPAQUE, TRANSPARENT},
    Position,
};

use super::{QuadGroups, Voxel};

pub const CHUNK_SIZE: f32 = 48.0;

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

    pub fn is_empty(&self) -> bool {
        self.voxels.iter().all(|voxel| *voxel == Voxel::Empty)
    }

    pub fn populate(&mut self, seed: u32) {
        let perlin = Perlin::new(seed);
        for i in 0..ChunkMesh::size() {
            let (local_x, local_y, local_z) = ChunkMesh::delinearize(i);

            let (x, y, z) = (
                local_x as i32 + &self.position.x * CHUNK_SIZE as i32,
                local_y as i32 + &self.position.y * CHUNK_SIZE as i32,
                local_z as i32 + &self.position.z * CHUNK_SIZE as i32,
            );

            let scale1 = 0.1;
            let scale2 = 0.01;
            let scale3 = 0.001;

            let val = perlin.get([x as f64 * scale1, z as f64 * scale1])
                + perlin.get([x as f64 * scale2, z as f64 * scale2])
                + perlin.get([x as f64 * scale3, z as f64 * scale3]);

            let val = val * 5.0;

            if y <= val as i32 {
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

    pub fn generate_mesh(&self) -> QuadGroups {
        let buffer = Arc::new(Mutex::new(QuadGroups::default()));

        (0..ChunkMesh::size()).into_par_iter().for_each(|i| {
            let (x, y, z) = ChunkMesh::delinearize(i);
            if (x > 0 && x < ChunkMesh::X - 1)
                && (y > 0 && y < ChunkMesh::Y - 1)
                && (z > 0 && z < ChunkMesh::Z - 1)
            {
                let voxel = self.get(x, y, z);

                match voxel.visibility() {
                    Visibility::Empty => return,
                    visibility => {
                        let neighbors = [
                            self.get(x - 1, y, z),
                            self.get(x + 1, y, z),
                            self.get(x, y - 1, z),
                            self.get(x, y + 1, z),
                            self.get(x, y, z - 1),
                            self.get(x, y, z + 1),
                        ];

                        neighbors
                            .into_par_iter()
                            .enumerate()
                            .for_each(|(i, neighbor)| {
                                let other = neighbor.visibility();

                                let generate = match (visibility, other) {
                                    (OPAQUE, EMPTY)
                                    | (OPAQUE, TRANSPARENT)
                                    | (TRANSPARENT, EMPTY) => true,

                                    (TRANSPARENT, TRANSPARENT) => voxel != neighbor,

                                    (_, _) => false,
                                };

                                match voxel {
                                    Voxel::Opaque(color) | Voxel::Transparent(color) => {
                                        if generate {
                                            let mut buffer = buffer.lock().unwrap();
                                            buffer.groups[i].push(Quad {
                                                voxel: [x, y, z],
                                                color,
                                            });
                                        }
                                    }
                                    _ => {}
                                }
                            });
                    }
                }
            }
        });

        let mut out = QuadGroups::default();
        out.groups = buffer.lock().unwrap().groups.clone();

        out
    }
}
