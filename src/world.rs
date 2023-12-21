pub mod despawn;
pub mod render_distance;

use bevy::app::{App, Plugin, Update};
pub use despawn::{despawn_handler, Despawn};
pub use render_distance::{render_distance_handler, RENDER_DISTANCE};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_handler)
            .add_systems(Update, render_distance_handler);
    }
}
