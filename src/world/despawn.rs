use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

#[derive(Component)]
pub struct Despawn;

pub fn despawn_handler(mut commands: Commands, mut despawns: Query<Entity, With<Despawn>>) {
    for entity in despawns.iter_mut() {
        commands.entity(entity).despawn();
    }
}
