use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;

mod camera;
mod config;
mod game;
mod hud;
mod logic;
mod scene;
mod schedule;

use camera::CameraPlugin;
use game::GamePlugin;
use hud::HudPlugin;
use scene::ScenePlugin;
use schedule::SchedulePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config::WINDOW_TITLE.to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::continuous())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins((
            SchedulePlugin,
            GamePlugin,
            ScenePlugin,
            CameraPlugin,
            HudPlugin,
        ))
        .run();
}
