use bevy::prelude::*;

mod animation;
pub mod camera;
pub mod level;
mod movement;
mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        camera::plugin,
        movement::plugin,
        player::plugin,
        level::plugin,
    ));
}
