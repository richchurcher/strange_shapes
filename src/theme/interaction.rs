use bevy::prelude::*;

use crate::{asset_tracking::AudioAssets, audio::SoundEffect, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(
        Update,
        (
            trigger_on_press,
            apply_interaction_palette,
            trigger_interaction_sound_effect,
        )
            .run_if(in_state(Screen::Title).or_else(in_state(Screen::Credits))),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(Event)]
pub struct OnPress;

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
        }
    }
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn trigger_interaction_sound_effect(
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        let source = match interaction {
            Interaction::Hovered => audio_assets.button_hover.clone(),
            Interaction::Pressed => audio_assets.button_press.clone(),
            _ => continue,
        };
        commands.spawn((
            AudioBundle {
                source,
                settings: PlaybackSettings::DESPAWN,
            },
            SoundEffect,
        ));
    }
}
