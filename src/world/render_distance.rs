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

pub const RENDER_DISTANCE: i32 = 12;

pub fn render_distance_handler(
    query: Query<&Transform, With<FlyCam>>,
    mut chunk_load_event: EventWriter<ChunkLoadEvent>,
    mut chunk_unload_event: EventWriter<ChunkUnloadEvent>,
    loaded_chunks: Res<LoadedChunks>,
) {
    let transform = query.get_single().expect("There should be a camera");

    let (chunk_x, chunk_y, chunk_z) = (
        (transform.translation.x / CHUNK_SIZE).floor() as i32,
        (transform.translation.y / CHUNK_SIZE).floor() as i32,
        (transform.translation.z / CHUNK_SIZE).floor() as i32,
    );

    let mut position = HashSet::new();

    for x in chunk_x - RENDER_DISTANCE..=chunk_x + RENDER_DISTANCE {
        for y in chunk_y - RENDER_DISTANCE..=chunk_y + RENDER_DISTANCE {
            for z in chunk_z - RENDER_DISTANCE..=chunk_z + RENDER_DISTANCE {
                let chunk_position = Position { x, y, z };
                position.insert(chunk_position);
                if !loaded_chunks.0.contains_key(&chunk_position) {
                    chunk_load_event.send(ChunkLoadEvent {
                        position: chunk_position,
                    });
                }
            }
        }
    }

    for (chunk_position, _) in loaded_chunks.0.iter() {
        if !position.contains(chunk_position) {
            chunk_unload_event.send(ChunkUnloadEvent {
                position: *chunk_position,
            });
        }
    }
}
