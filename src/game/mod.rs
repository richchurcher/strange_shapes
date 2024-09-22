use bevy::prelude::*;

mod animation;
pub mod camera;
pub mod level;
mod movement;
mod ship;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        movement::plugin,
        camera::plugin,
        level::plugin,
        ship::plugin,
    ));
}
