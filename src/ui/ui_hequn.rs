use bevy::prelude::*;
use bevy_egui::{egui::{self, Color32}, EguiContexts};

use crate::{general::Board, hequn::{game::HequnGame, general::HequnStep}, ui::ui_menu::UiMenuState};

pub fn ui_hequn(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    mut q_hequn: Query<&mut HequnGame>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let Ok(mut hequn) = q_hequn.single_mut() else {
        return Ok(())
    };

    egui::Window::new("Hequn")
        .open(&mut ui_menu.hequn_window_open)
        .show(ctx, |ui| {
            ui.label(hequn.board.game_info());

            ui.horizontal(|ui| {
                ui.colored_label(Color32::from_rgb(249, 106, 226), "■");
                ui.label(format!("Black: {}", hequn.board.black_score));
            });

            ui.horizontal(|ui| {
                ui.colored_label(Color32::from_rgb(127, 246, 244), "■");
                ui.label(format!("White: {}", hequn.board.white_score));
            });

            if ui.button("Pass").clicked() {
                hequn.try_move(HequnStep::Pass);
            }
        });

    Ok(())
}