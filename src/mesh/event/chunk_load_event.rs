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
    mut commands: Commands, // Commands for spawning entities and components
    mut chunk_load_event: EventReader<ChunkLoadEvent>, // Reader for `ChunkLoadEvent` events
    mut loaded_chunks: ResMut<LoadedChunks>, // Mutable reference to `LoadedChunks` resource
) {
    let thread_pool = AsyncComputeTaskPool::get(); // Get the async compute task pool

    // Iterate over each `ChunkLoadEvent` event
    for event in chunk_load_event.read() {
        let mut chunk_mesh = ChunkMesh::new(event.position); // Create a new `ChunkMesh` at the event position
        let task = thread_pool.spawn(async move {
            // Spawn a new task in the async compute task pool
            chunk_mesh.populate(1); // Populate the chunk mesh

            let result = chunk_mesh.generate_mesh(); // Generate the mesh for the chunk

            // If the result is empty, return early with the chunk position
            if result.is_empty() {
                return (None, chunk_mesh.position);
            }

            // Create new thread-safe vectors for positions, indices, normals, and colors
            let positions = Arc::new(Mutex::new(Vec::new()));
            let indices = Arc::new(Mutex::new(Vec::new()));
            let normals = Arc::new(Mutex::new(Vec::new()));
            let colors = Arc::new(Mutex::new(Vec::new()));

            // For each face in the result, extend the positions, indices, normals, and colors vectors
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

            // Create a new mesh with `PrimitiveTopology::TriangleList`
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

            // Unwrap the thread-safe vectors and insert them into the mesh
            let indices = Arc::try_unwrap(indices).unwrap().into_inner().unwrap();
            let positions = Arc::try_unwrap(positions).unwrap().into_inner().unwrap();
            let normals = Arc::try_unwrap(normals).unwrap().into_inner().unwrap();
            let colors = Arc::try_unwrap(colors).unwrap().into_inner().unwrap();

            mesh.set_indices(Some(Indices::U32(indices)));
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

            // Return the mesh and the chunk position
            (Some(mesh), chunk_mesh.position)
        });

        // Spawn a `ComputeTransform` entity with the task and insert it into the `loaded_chunks` resource
        let id = commands.spawn(ComputeTransform(task)).id();
        loaded_chunks.0.insert(event.position, id);
    }
}
