use std::os::windows;

use bevy::prelude::Component;
use bevy::{prelude::*, transform::commands};
use bevy_flycam::prelude::*;

mod voxel;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
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
    #[derive(Component)]
    struct Cursor;
    let cursor_texture_handle = asset_server.load("src/assets/cursor.png");

    commands.spawn(SpriteBundle {
        texture: cursor_texture_handle.into(),
        ..Default::default()
    })
    .insert(Cursor);
    
}
