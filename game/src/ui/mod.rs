mod time;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(EguiPrimaryContextPass, time::time_ui);
    }
}

fn init(mut egui_context: EguiContexts) {
    let ctx = egui_context.ctx_mut().expect("Could not get egui context");
    egui_extras::install_image_loaders(ctx);
}
