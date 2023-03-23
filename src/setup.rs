use bevy::{prelude::{
    default, Assets, Camera3dBundle, Commands, Mesh,
    ResMut, Transform, Vec3, StandardMaterial, PbrBundle, shape, Color, DirectionalLightBundle, DirectionalLight,
}};
use bevy_atmosphere::prelude::AtmosphereCamera;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

use std::io::{stdout, Write};

// setup for 3D scene
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mut lock = stdout().lock();
    
    let plane_handle = meshes.add(shape::Plane {size: 100.0,subdivisions: 16}.into());

    writeln!(lock, "hello world").unwrap();
    writeln!(lock, "{plane_handle:?}").unwrap();

    // terrain
    commands.spawn(PbrBundle {
        mesh: plane_handle,
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    
    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands
        .spawn((Camera3dBundle::default(), AtmosphereCamera::default()))
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(0.0, 45.0, 45.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}