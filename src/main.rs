use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod ui;

// use self::ui::UiCustomPlugin;
#[derive(Debug, Eq, Clone, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    // InGame,
    Paused,
    // End,
}

#[derive(Component)]
pub struct MainEntities;

#[derive(Component)]
pub struct MainCamera;

fn main() {
    let mut app = App::new();

    app.insert_state(AppState::MainMenu);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(500., 500.),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_plugins(WorldInspectorPlugin::new());

    // app.add_plugins(UiCustomPlugin);

    app.add_systems(Startup, (setup, pausing_state_handler));

    app.add_systems(OnExit(AppState::MainMenu), deliting_main_entities);

    app.add_systems(Update, paused_camera.run_if(in_state(AppState::Paused)));

    // app.add_systems(Update, bevy::window::close_on_esc);
    app.run();
}

fn setup(
    mut c: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    c.spawn((Camera2dBundle::default(), MainCamera, MainEntities));
    c.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh.add(Rectangle::new(100., 100.))),
            material: material.add(Color::BLUE),
            ..Default::default()
        },
        MainEntities,
    ));
}

fn deliting_main_entities(mut c: Commands, q: Query<Entity, With<MainEntities>>) {
    for entity in q.iter() {
        c.entity(entity).despawn();
    }
}

fn paused_camera(
    mut c: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    c.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::BLUE),
            ..Default::default()
        },
        ..Default::default()
    });
    c.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(mesh.add(Rectangle::new(100., 100.))),
        material: material.add(Color::GOLD),
        ..Default::default()
    },));
}

fn pausing_state_handler(
    mut state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut c: Commands,
) {
    if input.just_pressed(KeyCode::Escape) {
        match state.get() {
            AppState::MainMenu => next_state.set(AppState::Paused),
            AppState::Paused => (),
        }
    }
}
