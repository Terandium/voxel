pub mod mesh;

use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use mesh::{mesh_loader, mesh_unloader, mesher, ChunkLoadEvent, ChunkUnloadEvent};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const CHUNK_SIZE: f32 = 24.0;
const VOXEL_SIZE: f32 = 1.0;

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component, Eq, PartialEq)]
pub struct Chunk {
    pub position: Position,
}

impl Chunk {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

#[derive(Resource, Default)]
pub struct LoadedChunks(pub Vec<Chunk>);
fn main() {
    App::new()
        .init_resource::<LoadedChunks>()
        .add_event::<ChunkLoadEvent>()
        .add_event::<ChunkUnloadEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Voxel".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(MovementSettings {
            sensitivity: 0.00012,
            speed: 75.0,
        })
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, mesh_loader)
        .add_systems(Update, mesh_unloader)
        .add_systems(Update, mesher)
        .add_systems(Update, test)
        .add_systems(Update, fps_text_update_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.5),
            ..default()
        },
        FlyCam,
    ));

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 20000.0,
            color: Color::rgb(253.0 / 255.0, 251.0 / 255.0, 211.0 / 255.0),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        FpsText,
        TextBundle {
            // use two sections, so it is easy to update just the number
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                },
                TextSection {
                    value: " N/A".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                },
            ]),
            ..Default::default()
        },
    ));
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

fn test(
    query: Query<&Transform, With<FlyCam>>,
    mut chunk_load_event: EventWriter<ChunkLoadEvent>,
    mut chunk_unload_event: EventWriter<ChunkUnloadEvent>,
    loaded_chunks: Res<LoadedChunks>,
) {
    let transform = query.get_single().expect("There should be a camera");

    let (x, y, z) = transform.translation.into();
    let (chunk_x, chunk_y, chunk_z) = (
        (x / CHUNK_SIZE).floor() as i32,
        (y / CHUNK_SIZE).floor() as i32,
        (z / CHUNK_SIZE).floor() as i32,
    );

    let radius = 6;

    let position = Arc::new(Mutex::new(Vec::new()));

    (chunk_x - radius..=chunk_x + radius)
        .into_par_iter()
        .for_each(|x| {
            (chunk_y - radius..=chunk_y + radius)
                .into_par_iter()
                .for_each(|y| {
                    (chunk_z - radius..=chunk_z + radius)
                        .into_par_iter()
                        .for_each(|z| {
                            let chunk_position = Position { x, y, z };
                            position.lock().unwrap().push(chunk_position);
                        });
                });
        });

    for chunk in loaded_chunks.0.iter() {
        if !position.lock().unwrap().contains(&chunk.position) {
            chunk_unload_event.send(ChunkUnloadEvent {
                position: chunk.position,
            });
        }
    }

    for position in position.lock().unwrap().iter() {
        if !loaded_chunks.0.contains(&Chunk::new(*position)) {
            chunk_load_event.send(ChunkLoadEvent {
                position: *position,
            });
        }
    }
}
