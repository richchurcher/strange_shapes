mod asset_tracking;
pub mod audio;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screens;
mod theme;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use screens::Screen;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(Update, (AppSet::TickTimers, AppSet::Update).chain());
        app.configure_sets(
            Update,
            (AppSet::RecordInput, AppSet::PlayingUpdate)
                .chain()
                .run_if(in_state(Screen::Playing)),
        );

        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "strange shapes".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resizable: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        app.add_plugins((
            asset_tracking::plugin,
            game::plugin,
            screens::plugin,
            theme::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    TickTimers,
    RecordInput,
    Update,
    PlayingUpdate,
}
