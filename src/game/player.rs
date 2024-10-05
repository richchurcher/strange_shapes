use bevy::prelude::*;

use crate::{asset_tracking::SceneAssets, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), init);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn init(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        Name::new("Player"),
        Player,
        SceneBundle {
            scene: scene_assets.player.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}
