use bevy::prelude::*;

use crate::{game::movement::MovementController, screens::Screen, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, init);
    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSet::RecordInput),
    );
}

fn init(mut commands: Commands) {
    commands.spawn((
        Name::new("3D Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("Point light"),
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(5.0, 5.0, 0.0)),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Camera>>,
) {
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
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
