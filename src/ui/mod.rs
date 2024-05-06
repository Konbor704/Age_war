use bevy::prelude::*;
pub mod bottombar;
// pub mod topbar;

use self::{bottombar::UiBottomBarPlugin /* topbar::UiTopBarPlugin */};

pub struct UiCustomPlugin;

impl Plugin for UiCustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiBottomBarPlugin /* UiTopBarPlugin */,));
    }
}
