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
    mut commands: Commands, // Commands for spawning entities and components
    mut chunk_unload_event: EventReader<ChunkUnloadEvent>, // Reader for `ChunkUnloadEvent` events
    mut loaded_chunks: ResMut<LoadedChunks>, // Mutable reference to `LoadedChunks` resource
) {
    // Iterate over each `ChunkUnloadEvent` event
    for event in chunk_unload_event.read() {
        // If the `LoadedChunks` resource contains the event position, remove it
        if let Some(entity) = loaded_chunks.0.remove(&event.position) {
            // If the entity exists, insert a `Despawn` component
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.try_insert(Despawn);
            }
        }
    }
}
