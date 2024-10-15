use bevy::prelude::*;

use crate::{asset_tracking::SceneAssets, screens::Screen, AppSet};

use super::movement::MovementController;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), init);
    app.add_systems(Update, player_movement.in_set(AppSet::PlayingUpdate));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn init(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    scene_assets: Res<SceneAssets>,
) {
    commands.spawn((
        Name::new("Player"),
        MovementController::default(),
        Player,
        SceneBundle {
            scene: scene_assets.player.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 0.0)),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        Name::new("Floor"),
        PbrBundle {
            mesh: meshes.add(Circle::new(40.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut intent = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.z -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.z += 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}
