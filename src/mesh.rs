pub mod chunk;
pub mod event;
pub mod face;
pub mod generation;
pub mod quad;
pub mod side;
pub mod voxel;

use bevy::app::{App, Plugin, Update};
pub use chunk::{Chunk, ChunkMesh, LoadedChunks, CHUNK_SIZE};
pub use event::{
    chunk_load_event_handler, chunk_unload_event_handler, ChunkLoadEvent, ChunkUnloadEvent,
};
pub use face::Face;
pub use generation::mesher;
pub use quad::{Quad, QuadGroups};
pub use side::{Axis, Side};
pub use voxel::{Voxel, VOXEL_SIZE};

pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedChunks>()
            .add_event::<ChunkLoadEvent>()
            .add_event::<ChunkUnloadEvent>()
            .add_systems(Update, chunk_load_event_handler)
            .add_systems(Update, chunk_unload_event_handler)
            .add_systems(Update, mesher);
    }
}
