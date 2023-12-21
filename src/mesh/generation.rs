use futures_lite::future;
use std::sync::{Arc, Mutex};

use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        event::EventReader,
        system::{Commands, Query, ResMut},
    },
    math::Vec3,
    pbr::{PbrBundle, StandardMaterial},
    render::{
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    tasks::{block_on, AsyncComputeTaskPool, Task},
    transform::components::Transform,
};
use rayon::iter::ParallelIterator;

use crate::{Chunk, LoadedChunks, Position, CHUNK_SIZE, VOXEL_SIZE};

use super::{ChunkLoadEvent, ChunkMesh, ChunkUnloadEvent};

#[derive(bevy::ecs::component::Component)]
pub struct ComputeTransform(Task<(Mesh, Position)>);

pub fn mesher(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ComputeTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loaded_chunks: ResMut<LoadedChunks>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some((mesh, position)) = block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<ComputeTransform>();
            let entity = commands.get_entity(entity);

            if let Some(mut entity) = entity {
                let id = entity
                    .try_insert(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(StandardMaterial::default()),
                        transform: Transform::from_translation(Vec3::new(
                            position.x as f32 * CHUNK_SIZE,
                            position.y as f32 * CHUNK_SIZE,
                            position.z as f32 * CHUNK_SIZE,
                        )),
                        ..Default::default()
                    })
                    .try_insert(Chunk::new(position))
                    .id();

                loaded_chunks.0.insert(position, id);
            }
        }
    }
}

pub fn mesh_loader(
    mut commands: Commands,
    mut chunk_load_event: EventReader<ChunkLoadEvent>,
    mut loaded_chunks: ResMut<LoadedChunks>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in chunk_load_event.read() {
        let mut chunk_mesh = ChunkMesh::new(event.position);
        let task = thread_pool.spawn(async move {
            chunk_mesh.populate(1);

            let result = chunk_mesh.generate_mesh();

            let positions = Arc::new(Mutex::new(Vec::new()));
            let indices = Arc::new(Mutex::new(Vec::new()));
            let normals = Arc::new(Mutex::new(Vec::new()));
            let colors = Arc::new(Mutex::new(Vec::new()));

            result.iter().for_each(|face| {
                let mut positions = positions.lock().unwrap();
                let mut indices = indices.lock().unwrap();
                let mut normals = normals.lock().unwrap();
                let mut colors = colors.lock().unwrap();

                indices.extend_from_slice(&face.indices(positions.len() as u32));
                positions.extend_from_slice(&face.positions(VOXEL_SIZE));
                normals.extend_from_slice(&face.normals());
                colors.extend_from_slice(&face.colors());
            });

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

            let indices = Arc::try_unwrap(indices).unwrap().into_inner().unwrap();
            let positions = Arc::try_unwrap(positions).unwrap().into_inner().unwrap();
            let normals = Arc::try_unwrap(normals).unwrap().into_inner().unwrap();
            let colors = Arc::try_unwrap(colors).unwrap().into_inner().unwrap();

            mesh.set_indices(Some(Indices::U32(indices)));
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

            (mesh, chunk_mesh.position)
        });

        let id = commands.spawn(ComputeTransform(task)).id();
        loaded_chunks.0.insert(event.position, id);
    }
}

pub fn mesh_unloader(
    mut commands: Commands,
    mut chunk_unload_event: EventReader<ChunkUnloadEvent>,
    mut loaded_chunks: ResMut<LoadedChunks>,
) {
    for event in chunk_unload_event.read() {
        if let Some(entity) = loaded_chunks.0.remove(&event.position) {
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.despawn();
            }
        } else {
            panic!("Chunk not loaded");
        }
    }
}
