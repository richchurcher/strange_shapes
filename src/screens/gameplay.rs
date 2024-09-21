//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    asset_tracking::AudioAssets, audio::Music, demo::level::spawn_level as spawn_level_command,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);

    app.add_systems(OnEnter(Screen::Gameplay), play_gameplay_music);
    app.add_systems(OnExit(Screen::Gameplay), stop_music);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {
    commands.add(spawn_level_command);
}

fn play_gameplay_music(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        AudioBundle {
            source: audio_assets.fluffing_a_duck.clone(),
            settings: PlaybackSettings::LOOP,
        },
        Music,
    ));
}

fn stop_music(mut commands: Commands, music: Query<Entity, With<Music>>) {
    if let Ok(entity) = music.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
