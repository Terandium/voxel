use bevy::ecs::{
    event::{Event, EventReader},
    system::{Commands, ResMut},
};

use crate::{util::Position, world::Despawn, LoadedChunks};

#[derive(Event)]
pub struct ChunkUnloadEvent {
    pub position: Position,
}

pub fn chunk_unload_event_handler(
    mut commands: Commands,
    mut chunk_unload_event: EventReader<ChunkUnloadEvent>,
    mut loaded_chunks: ResMut<LoadedChunks>,
) {
    for event in chunk_unload_event.read() {
        if let Some(entity) = loaded_chunks.0.remove(&event.position) {
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.try_insert(Despawn);
            }
        }
    }
}
