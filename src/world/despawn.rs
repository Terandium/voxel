use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

#[derive(Component)]
pub struct Despawn;

pub fn despawn_handler(
    mut commands: Commands, // Commands for spawning entities and components
    mut despawns: Query<Entity, With<Despawn>>, // Query for entities with a `Despawn` component
) {
    // Iterate over each entity in the query
    for entity in despawns.iter_mut() {
        // Despawn the entity
        commands.entity(entity).despawn();
    }
}
