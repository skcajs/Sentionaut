mod setup;

use bevy::prelude::{App, DefaultPlugins};
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};
use bevy_editor_pls::prelude::*;
use bevy_atmosphere::prelude::*;

use setup::setup_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(AtmospherePlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_startup_system(setup_world)
        .run();
}
