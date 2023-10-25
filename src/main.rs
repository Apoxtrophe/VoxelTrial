use std::os::windows;
use bevy::render::view::{ExtractedWindows, WindowSurfaces};
use bevy::window::Window;

use bevy::input::mouse::MouseMotion;
use bevy::input::Input;

use bevy::prelude::Component;
use bevy::{prelude::*, transform::commands};
use bevy_flycam::prelude::*;
mod voxel;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, cursor_follow_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: Res<AssetServer>
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    //Add cursor
   
    let cursor_texture_handle = asset_server.load("cursor.png");

    commands.spawn(SpriteBundle {
        texture: cursor_texture_handle.into(),
        ..Default::default()
    })
    .insert(Cursor);
    
}

#[derive(Component)]
struct Cursor;

fn cursor_follow_system(
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    windows: Query<&Window>,
) {
    for window in windows.iter() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let cursor_position = window_size / 2.0;

        for mut transform in cursor_query.iter_mut() {
            transform.translation = Vec3::new(cursor_position.x, cursor_position.y, 0.0);
        }
    }
}
