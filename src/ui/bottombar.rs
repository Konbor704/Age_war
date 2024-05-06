use bevy::prelude::*;

pub struct UiBottomBarPlugin;
impl Plugin for UiBottomBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_bottom_bar);
    }
}

#[derive(Component)]
struct BottomBar;

pub fn ui_bottom_bar(mut c: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("bottombar.png");

    c.spawn(NodeBundle {
        style: Style {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.)],
            grid_template_rows: vec![GridTrack::auto(), GridTrack::flex(1.0), GridTrack::px(200.)],
            ..Default::default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..Default::default()
    })
    .with_children(|builder| {
        builder.spawn(ImageBundle {
            style: Style {
                display: Display::Grid,
                // grid_column: GridPlacement::span(2),
                grid_row: GridPlacement::span(3),
                ..Default::default()
            },
            image: UiImage::new(image),
            ..Default::default()
        });
    });
}
