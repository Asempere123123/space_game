mod time;

use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, time::time_ui);
    }
}

fn init(mut egui_context: EguiContexts) {
    let ctx = egui_context.ctx_mut();
    egui_extras::install_image_loaders(ctx);
}
