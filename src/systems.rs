use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::components::*;
use crate::AppState;

pub fn setup(
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

pub fn deliting_main_entities(mut c: Commands, q: Query<Entity, With<MainEntities>>) {
    for entity in q.iter() {
        c.entity(entity).despawn();
    }
}

pub fn paused_camera(
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

pub fn pausing_state_handler(
    mut next_state: ResMut<NextState<AppState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Paused);
    }
}
