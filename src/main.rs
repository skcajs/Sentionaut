mod setup;

use bevy::{
    prelude::{App, AmbientLight, Color, ClearColor, MaterialPlugin, DefaultPlugins, Material, Mesh}, 
    render::{
        mesh::{
            Indices, PrimitiveTopology,
            VertexAttributeValues,
        },
        render_resource::{AsBindGroup, ShaderRef},
    }, reflect::TypeUuid
};

use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};
use bevy_editor_pls::prelude::*;
use bevy_atmosphere::prelude::*;

use setup::setup_world;

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
        .add_plugin(EditorPlugin)
        .add_plugin(AtmospherePlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_startup_system(setup_world)
        .run();
}

