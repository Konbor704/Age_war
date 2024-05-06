use bevy::{prelude::*, window::WindowResolution};

mod ui;

use self::ui::UiCustomPlugin;

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

    app.add_plugins(UiCustomPlugin);

    app.add_systems(Startup, setup);

    app.add_systems(Update, bevy::window::close_on_esc);
    app.run();
}

fn setup(mut c: Commands) {
    c.spawn(Camera2dBundle::default());
}