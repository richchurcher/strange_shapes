use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{game::level::spawn_level as spawn_level_command, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_level);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {
    commands.add(spawn_level_command);
    // commands.spawn((
    //     AudioBundle {
    //         source: audio_assets.fluffing_a_duck.clone(),
    //         settings: PlaybackSettings::LOOP,
    //     },
    //     Music,
    //     StateScoped(Screen::Playing),
    // ));
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
