use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{
            Indices, PrimitiveTopology,
            VertexAttributeValues,
        },
        render_resource::{AsBindGroup, ShaderRef},
    }
};
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(ClearColor(
            Color::hex("590059").unwrap(),
        ))
        .add_plugins(DefaultPlugins)
        .add_plugin(
            MaterialPlugin::<LandMaterial>::default(),
        )
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LandMaterial>>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.5, 2.0)
                .looking_at(
                    Vec3::new(0.0, 1.5, 0.0),
                    Vec3::Y,
                ),
            ..default()
        });

    // land
    let mut land = Mesh::from(Land {
        size: 1000.0,
        num_vertices: 1000,
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
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for LandMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/land_vertex_shader.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct LandMaterial {
    #[uniform(0)]
    time: f32,
}

#[derive(Debug, Copy, Clone)]
struct Land {
    size: f32,
    num_vertices: u32,
}

impl From<Land> for Mesh {
    fn from(plane: Land) -> Self {
        let extent = plane.size / 2.0;

        let jump = extent / plane.num_vertices as f32;

        let vertices = (0..=plane.num_vertices)
            .cartesian_product(0..=plane.num_vertices)
            .map(|(y, x)| {
                (
                    [
                        x as f32 * jump - 0.5 * extent,
                        0.0,
                        y as f32 * jump - 0.5 * extent,
                    ],
                    [0.0, 1.0, 0.0],
                    [
                        x as f32
                            / plane.num_vertices as f32,
                        y as f32
                            / plane.num_vertices as f32,
                    ],
                )
            })
            .collect::<Vec<_>>();

        let indices = Indices::U32(
            (0..=plane.num_vertices)
                .cartesian_product(0..=plane.num_vertices)
                .enumerate()
                .filter_map(|(index, (x, y))| {
                    if y >= plane.num_vertices {
                        None
                    } else if x >= plane.num_vertices {
                        None
                    } else {
                        Some([
                            [
                                index as u32,
                                index as u32
                                    + 1
                                    + 1
                                    + plane.num_vertices,
                                index as u32 + 1,
                            ],
                            [
                                index as u32,
                                index as u32
                                    + 1
                                    + plane.num_vertices,
                                index as u32
                                    + plane.num_vertices
                                    + 1
                                    + 1,
                            ],
                        ])
                    }
                })
                .flatten()
                .flatten()
                .collect::<Vec<_>>(),
        );

        let positions: Vec<_> =
            vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> =
            vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> =
            vertices.iter().map(|(_, _, uv)| *uv).collect();

        let mut mesh =
            Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions,
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}