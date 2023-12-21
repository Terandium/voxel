use std::sync::{Arc, Mutex};

use bevy::{
    ecs::{
        event::{Event, EventReader},
        system::{Commands, ResMut},
    },
    render::{
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    tasks::AsyncComputeTaskPool,
};
use rayon::iter::ParallelIterator;

use crate::{
    mesh::{generation::ComputeTransform, ChunkMesh},
    util::Position,
    LoadedChunks,
};

#[derive(Event)]
pub struct ChunkLoadEvent {
    pub position: Position,
}

pub fn chunk_load_event_handler(
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

            if result.is_empty() {
                return (None, chunk_mesh.position);
            }

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
                positions.extend_from_slice(&face.positions());
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

            (Some(mesh), chunk_mesh.position)
        });

        let id = commands.spawn(ComputeTransform(task)).id();
        loaded_chunks.0.insert(event.position, id);
    }
}
