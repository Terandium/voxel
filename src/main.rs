pub mod mesh;
pub mod util;
pub mod world;

use std::f32::consts::PI;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_flycam::PlayerPlugin;
use mesh::{LoadedChunks, MeshPlugin};
use util::Position;
use world::{despawn_handler, render_distance_handler};

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Voxel".to_string(),
                present_mode: PresentMode::AutoVsync,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(MeshPlugin)
        .add_systems(Update, despawn_handler)
        .add_systems(Update, render_distance_handler)
        .add_systems(Update, fps_text_update_system)
        .add_systems(Startup, setup)
        .run();
}

//todo: all of this below will be removed eventually anyways so no need to move it to a different file

fn setup(mut commands: Commands) {
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
