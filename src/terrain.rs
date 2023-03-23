use bevy::{prelude::{Mesh, Material}, render::{mesh::Indices, render_resource::{AsBindGroup, ShaderRef, PrimitiveTopology}}, reflect::TypeUuid};
use itertools::Itertools;

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
    pub time: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Land {
    pub size: f32,
    pub num_vertices: u32,
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
                    ],  // increments from -x to +x, e.g -5 to +5
                    [0.0, 1.0, 0.0], // Normals
                    [
                        x as f32
                            / plane.num_vertices as f32,
                        y as f32
                            / plane.num_vertices as f32,
                    ],
                )
            })
            .collect::<Vec<_>>();

        // Creating the triangles
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