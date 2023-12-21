use futures_lite::future;

use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::Vec3,
    pbr::{PbrBundle, StandardMaterial},
    render::mesh::Mesh,
    tasks::{block_on, Task},
    transform::components::Transform,
};

use crate::{world::Despawn, Position};

use super::{Chunk, CHUNK_SIZE};

#[derive(bevy::ecs::component::Component)]
pub struct ComputeTransform(pub Task<(Option<Mesh>, Position)>);

pub fn mesher(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ComputeTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some((mesh, position)) = block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<ComputeTransform>();
            let entity = commands.get_entity(entity);
            if let Some(mut entity) = entity {
                match mesh {
                    Some(mesh) => {
                        entity
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
                            .try_insert(Chunk::new(position));
                    }
                    None => {
                        entity.try_insert(Despawn);
                    }
                }
            }
        }
    }
}
