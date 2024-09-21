use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Title)
            .load_collection::<AudioAssets>()
            .load_collection::<ImageAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub button_hover: Handle<AudioSource>,

    #[asset(path = "audio/sound_effects/button_press.ogg")]
    pub button_press: Handle<AudioSource>,

    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    pub fluffing_a_duck: Handle<AudioSource>,

    #[asset(path = "audio/music/Monkeys Spinning Monkeys.ogg")]
    pub monkeys_spinning_monkeys: Handle<AudioSource>,

    #[asset(path = "audio/sound_effects/step1.ogg")]
    pub path_step_1: Handle<AudioSource>,

    #[asset(path = "audio/sound_effects/step2.ogg")]
    pub path_step_2: Handle<AudioSource>,

    #[asset(path = "audio/sound_effects/step3.ogg")]
    pub path_step_3: Handle<AudioSource>,

    #[asset(path = "audio/sound_effects/step4.ogg")]
    pub path_step_4: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/ducky.png")]
    pub ducky: Handle<Image>,
}
