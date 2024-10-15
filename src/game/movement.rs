use bevy::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();

    app.add_systems(
        Update,
        (apply_movement).chain().in_set(AppSet::PlayingUpdate),
    );
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub intent: Vec3,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics
    /// engine.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec3::ZERO,
            // 400 pixels per second is a nice default, but we can still vary this per character.
            max_speed: 10.0,
        }
    }
}

fn apply_movement(
    mut movement_query: Query<(&MovementController, &mut Transform)>,
    time: Res<Time>,
) {
    for (controller, mut transform) in &mut movement_query {
        dbg!(controller.intent);
        let velocity = controller.max_speed * controller.intent;
        transform.translation += velocity * time.delta_seconds();
    }
}
