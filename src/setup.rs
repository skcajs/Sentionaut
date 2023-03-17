use bevy::prelude::{
    default, shape, Assets, Camera3dBundle, Color, Commands, Mesh, PbrBundle, PointLight,
    PointLightBundle, ResMut, StandardMaterial, Transform, Vec3,
};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

// setup for 3D scene
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LandMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // // Plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });

    // Plane
    let mut land = Mesh::from(Land {
        size: 1000.0,
        num_vertices: 1000,
    });
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        land.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[r, g, b]| [(1. - *r) / 2., (1. - *g) / 2., (1. - *b) / 2., 1.])
            .collect();
        land.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    commands
        .spawn(MaterialMeshBundle::default())
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(land),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(LandMaterial {
                time: 0.,
                ship_position: Vec3::ZERO,
            }),
            // material: standard_materials.add(
            //     StandardMaterial {
            //         base_color: Color::WHITE,
            //         ..default()
            //     },
            // ),
            ..default()
        });
    // .insert(Wireframe);

    // ship
    commands
        .spawn(MaterialMeshBundle::default())
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            material: standard_materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            ..default()
        })
        .insert(Ship)
        .insert(Movable);

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}
