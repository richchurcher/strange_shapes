//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use crate::{asset_tracking::AudioAssets, audio::Music, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), spawn_credits_screen);
    app.add_systems(OnEnter(Screen::Credits), play_credits_music);
    app.add_systems(OnExit(Screen::Credits), stop_music);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.header("Made by");
            children.label("Joe Shmoe - Implemented aligator wrestling AI");
            children.label("Jane Doe - Made the music for the alien invasion");

            children.header("Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");
            children.label("Ducky sprite - CC0 by Caz Creates Games");
            children.label("Button SFX - CC0 by Jaszunio15");
            children.label("Music - CC BY 3.0 by Kevin MacLeod");

            children.button("Back").observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn play_credits_music(_commands: Commands, _audio_assets: Res<AudioAssets>) {
    // commands.spawn((
    //     AudioBundle {
    //         source: audio_assets.monkeys_spinning_monkeys.clone(),
    //         settings: PlaybackSettings::LOOP,
    //     },
    //     Music,
    // ));
}

fn stop_music(mut commands: Commands, music: Query<Entity, With<Music>>) {
    if let Ok(entity) = music.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
