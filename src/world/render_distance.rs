use std::collections::VecDeque;

use bevy::{
    ecs::{
        event::EventWriter,
        query::With,
        system::{Query, Res},
    },
    transform::components::Transform,
    utils::HashSet,
};
use bevy_flycam::FlyCam;

use crate::{
    mesh::{ChunkLoadEvent, ChunkUnloadEvent, LoadedChunks, CHUNK_SIZE},
    util::Position,
};

/// The render distance in chunks.
pub const RENDER_DISTANCE: i32 = 12;

// This system handles loading and unloading chunks based on the player's position.
// Uses a breadth-first search to find all chunks within the render distance.
pub fn render_distance_handler(
    // Query for the camera's transform and ensure it has a FlyCam component.
    query: Query<&Transform, With<FlyCam>>,
    // Event writer for chunk load events.
    mut chunk_load_event: EventWriter<ChunkLoadEvent>,
    // Event writer for chunk unload events.
    mut chunk_unload_event: EventWriter<ChunkUnloadEvent>,
    // Resource containing currently loaded chunks.
    loaded_chunks: Res<LoadedChunks>,
) {
    // Get the camera's transform.
    let transform = query.get_single().expect("There should be a camera");

    // Calculate the chunk coordinates the camera is currently in.
    let (chunk_x, chunk_y, chunk_z) = (
        (transform.translation.x / CHUNK_SIZE).floor() as i32,
        (transform.translation.y / CHUNK_SIZE).floor() as i32,
        (transform.translation.z / CHUNK_SIZE).floor() as i32,
    );

    // Initialize a queue, a visited set, and a to_be_loaded set.
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut to_be_loaded = HashSet::new();

    // Create a Position struct for the player's position.
    let player_position = Position {
        x: chunk_x,
        y: chunk_y,
        z: chunk_z,
    };
    // Add the player's position to the queue and visited set.
    queue.push_back(player_position);
    visited.insert(player_position);

    // While there are positions in the queue...
    while let Some(position) = queue.pop_front() {
        // If the position is within the render distance...
        if position.distance(&player_position) <= RENDER_DISTANCE.into() {
            // Add the position to the to_be_loaded set.
            to_be_loaded.insert(position);
            // If the position is not in the loaded chunks...
            if !loaded_chunks.0.contains_key(&position) {
                // Send a chunk load event for the position.
                chunk_load_event.send(ChunkLoadEvent { position: position });
            }

            // For each neighbor of the position...
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let neighbor_position = Position {
                            x: position.x + dx,
                            y: position.y + dy,
                            z: position.z + dz,
                        };

                        // If the neighbor has not been visited...
                        if !visited.contains(&neighbor_position) {
                            // Add the neighbor to the queue and visited set.
                            queue.push_back(neighbor_position);
                            visited.insert(neighbor_position);
                        }
                    }
                }
            }
        }
    }

    // For each loaded chunk...
    for (chunk_position, _) in loaded_chunks.0.iter() {
        // If the chunk is not in the to_be_loaded set...
        if !to_be_loaded.contains(chunk_position) {
            // Send a chunk unload event for the chunk.
            chunk_unload_event.send(ChunkUnloadEvent {
                position: *chunk_position,
            });
        }
    }
}
