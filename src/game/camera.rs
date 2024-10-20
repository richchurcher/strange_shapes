use std::{f32::consts::FRAC_PI_2, ops::Range};

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{screens::Screen, AppSet};

use super::player::Player;

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            // Required to stop the camera sinking beneath the "floor"!
            pitch_range: -1.0..-0.1,
            yaw_speed: 0.004,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CameraSettings>();
    app.add_systems(Startup, init);
    app.add_systems(Update, orbit.in_set(AppSet::PlayingUpdate));
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
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(5.0, 5.0, 0.0)),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

fn orbit(
    mut camera: Query<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut transform = camera.single_mut();
    let mut delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        delta += event.delta;
    }

    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
    // as we are already receiving the full movement since the last frame was rendered. Multiplying
    // by delta time here would make the movement slower that it should be.
    let delta_pitch = delta.y * camera_settings.pitch_speed;
    let delta_yaw = delta.x * camera_settings.yaw_speed;

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
    );
    let yaw = yaw + delta_yaw;
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    let mut target = Vec3::ZERO;
    if let Ok(player_transform) = player.get_single() {
        target = player_transform.translation;
    }
    transform.translation = target - transform.forward() * camera_settings.orbit_distance;
}
