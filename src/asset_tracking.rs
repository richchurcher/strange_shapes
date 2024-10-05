use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Title)
            .load_collection::<AudioAssets>()
            .load_collection::<SceneAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub button_hover: Handle<AudioSource>,

    #[asset(path = "audio/sound_effects/button_press.ogg")]
    pub button_press: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "models/player.glb#Scene0")]
    pub player: Handle<Scene>,
}
