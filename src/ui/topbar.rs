// use bevy::prelude::*;
//
// pub struct UiTopBarPlugin;
// impl Plugin for UiTopBarPlugin {
//     fn build(&self, app: &mut App) {}
// }
//
// #[derive(Component)]
// pub struct TopBar;
//
// fn ui_top_bar(mut c: Commands, asset_server: Res<AssetServer>) {
//     c.spawn((
//         ImageBundle {
//             style: Style {
//                 width: Val::Percent(100.),
//                 flex_direction: FlexDirection::Column,
//                 justify_content: JustifyContent::FlexStart,
//                 align_items: AlignItems::Stretch,
//                 overflow: Overflow::clip_x(),
//                 ..Default::default()
//             },
//             image: UiImage::new(asset_server.load("bottombar.png")),
//             ..Default::default()
//         },
//         TopBar,
//     ));
// }
