use futures_lite::future;

use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
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

use super::{Chunk, CHUNK_SIZE, VOXEL_SIZE};

// Public struct `ComputeTransform` that wraps a `Task` which returns an `Option<Mesh>` and a `Position`.
#[derive(Component)]
pub struct ComputeTransform(pub Task<(Option<Mesh>, Position)>);

// Public function `mesher` that processes `ComputeTransform` tasks.
pub fn mesher(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ComputeTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Iterate over each `ComputeTransform` task.
    for (entity, mut task) in tasks.iter_mut() {
        // If the task is ready and returns a result,
        if let Some((mesh, position)) = block_on(future::poll_once(&mut task.0)) {
            // Remove the `ComputeTransform` component from the entity.
            commands.entity(entity).remove::<ComputeTransform>();
            // Get the entity from the commands.
            let entity = commands.get_entity(entity);
            // If the entity exists,
            if let Some(mut entity) = entity {
                // Depending on whether the mesh exists or not,
                match mesh {
                    // If the mesh exists,
                    Some(mesh) => {
                        // Insert a `PbrBundle` and a `Chunk` into the entity.
                        entity
                            .try_insert(PbrBundle {
                                mesh: meshes.add(mesh),
                                material: materials.add(StandardMaterial::default()),
                                transform: Transform::from_translation(Vec3::new(
                                    position.x as f32 * CHUNK_SIZE * VOXEL_SIZE,
                                    position.y as f32 * CHUNK_SIZE * VOXEL_SIZE,
                                    position.z as f32 * CHUNK_SIZE * VOXEL_SIZE,
                                )),
                                ..Default::default()
                            })
                            .try_insert(Chunk::new(position));
                    }
                    // If the mesh does not exist,
                    None => {
                        // Insert a `Despawn` into the entity.
                        entity.try_insert(Despawn);
                    }
                }
            }
        }
    }
}
