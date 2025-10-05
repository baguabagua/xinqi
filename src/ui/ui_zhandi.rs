use bevy::prelude::*;
use bevy_egui::{egui::{self, Color32}, EguiContexts};

use crate::{general::Board, zhandi::{game::ZhandiGame}, ui::ui_menu::UiMenuState};

pub fn ui_zhandi(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    q_zhandi: Query<&ZhandiGame>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let Ok(zhandi) = q_zhandi.single() else {
        return Ok(())
    };

    egui::Window::new("Zhandi")
        .open(&mut ui_menu.zhandi_window_open)
        .show(ctx, |ui| {
            ui.label(zhandi.board.game_info());

            ui.horizontal(|ui| {
                ui.colored_label(Color32::from_rgb(249, 106, 226), "■");
                ui.label(format!("Black: {}", zhandi.board.black_score));
            });

            ui.horizontal(|ui| {
                ui.colored_label(Color32::from_rgb(127, 246, 244), "■");
                ui.label(format!("White: {}", zhandi.board.white_score));
            });
        });

    Ok(())
}