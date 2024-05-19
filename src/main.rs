use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
mod ui;

use components::*;
use events::*;
use resources::*;
use systems::*;

// use self::ui::UiCustomPlugin;
#[derive(Debug, Eq, Clone, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    // InGame,
    Paused,
    // End,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(500., 500.),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.init_state::<AppState>();
    app.add_plugins(WorldInspectorPlugin::new());

    // app.add_plugins(UiCustomPlugin);

    app.add_systems(Startup, setup);
    app.add_systems(Update, pausing_state_handler);

    app.add_systems(OnExit(AppState::MainMenu), deliting_main_entities);

    app.add_systems(OnEnter(AppState::Paused), paused_camera);

    app.add_systems(Update, bevy::window::close_on_esc);
    app.run();
}
