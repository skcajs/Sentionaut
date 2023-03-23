use bevy::{prelude::{
    default, Assets, Camera3dBundle, Commands, Mesh, PointLight,
    PointLightBundle, ResMut, Transform, Vec3, MaterialMeshBundle,
}, render::mesh::VertexAttributeValues};
use bevy_atmosphere::prelude::AtmosphereCamera;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

use crate::terrain::{LandMaterial, Land};

// setup for 3D scene
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LandMaterial>>,
) {
    // land
    let mut land = Mesh::from(Land {
        size: 100.0,
        num_vertices: 10,
    });
    if let Some(VertexAttributeValues::Float32x3(
        positions,
    )) = land.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[r, g, b]| {
                [
                    (1. - *r) / 2.,
                    (1. - *g) / 2.,
                    (1. - *b) / 2.,
                    1.,
                ]
            })
            .collect();
        land.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            colors,
        );
    }

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(land),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(LandMaterial {
            time: 0.,
        }),
        ..default()
    });
    
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        // transform: Transform::from_xyz(4.0, 8.0, 4.0),
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