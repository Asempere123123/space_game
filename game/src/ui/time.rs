use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Vec2},
    EguiContexts,
};
use orbits::TimeSpeed;

const WARPS_AMMOUNT: usize = 14;
const WAPRPS: [f64; WARPS_AMMOUNT] = [
    1.0, 2.0, 3.0, 4.0, 10.0, 100.0, 1.0e3, 1.0e4, 1.0e5, 1.0e6, 1.0e7, 1.0e11, 1.0e14, 1.0e17,
];

pub fn time_ui(
    mut egui_context: EguiContexts,
    mut time_speed: ResMut<TimeSpeed>,
    mut enabled: Local<usize>,
) {
    let ctx = egui_context.ctx_mut();
    egui::Area::new(egui::Id::new("time"))
        .fixed_pos((10.0, 5.0))
        .show(ctx, |ui| {
            #[cfg(not(target_arch = "wasm32"))]
            egui::Image::new("file://assets/ui/time_background.png")
                .paint_at(ui, ui.max_rect().expand2(Vec2::new(10.0, 5.0)));
            #[cfg(target_arch = "wasm32")]
            egui::Image::new("https://space-game.asempere.net/assets/ui/time_background.png")
                .paint_at(ui, ui.max_rect().expand2(Vec2::new(10.0, 5.0)));
            ui.horizontal(|ui| {
                for (i, warp) in WAPRPS.iter().enumerate() {
                    let img_path = if *enabled == i {
                        if cfg!(target_arch = "wasm32") {
                            "https://space-game.asempere.net/assets/ui/timewarp_arrow_on.png"
                        } else {
                            "file://assets/ui/timewarp_arrow_on.png"
                        }
                    } else {
                        if cfg!(target_arch = "wasm32") {
                            "https://space-game.asempere.net/assets/ui/timewarp_arrow_off.png"
                        } else {
                            "file://assets/ui/timewarp_arrow_off.png"
                        }
                    };
                    let img = egui::Image::new(img_path).max_size(Vec2::new(20.0, 20.0));
                    let button = egui::ImageButton::new(img).frame(false);
                    if ui.add(button).clicked() {
                        *enabled = i;
                        time_speed.0 = *warp;
                    }
                }
            })
        });
}
