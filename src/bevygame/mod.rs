use crate::bevygame::bird::BirdPlugin;
use bevy::prelude::*;
use crate::bevygame::witch::WitchPlugin;
//use bevy_shader_utils::ShaderUtilsPlugin;

use self::{setup_res::ResourceSetupPlugin, slime::SlimePlugin};

mod witch;
mod bird;
mod setup_res;
mod slime;

pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "game".into(),
                        resolution: (393., 851.).into(),
                        // present_mode: PresentMode::AutoVsync,
                        // fit_canvas_to_parent: false,
                        // prevent_default_event_handling: false,
                        // window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                }),
            //ShaderUtilsPlugin,
            ResourceSetupPlugin,
            BirdPlugin,
            WitchPlugin,
            SlimePlugin
        ))
        .run();
}
