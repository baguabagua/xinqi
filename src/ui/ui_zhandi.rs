use bevy::prelude::*;
use bevy_egui::{egui::{self, Color32}, EguiContexts};

use crate::{ai::{mcts::MCTSAI, mctsv2::MCTSv2, AI}, general::Board, ui::ui_menu::UiMenuState, zhandi::{game::ZhandiGame, ai::*}};

pub fn ui_zhandi(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    mut q_zhandi: Query<&mut ZhandiGame>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let Ok(mut zhandi) = q_zhandi.single_mut() else {
        return Ok(())
    };

    let ai_time_limit_ms = ui_menu.ai_time_limit_ms;

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

            if ui.button("weak ai play").clicked() {
                let ai = MCTSAI::new();
                let ai_step = ai.play(zhandi.board.clone(), ai_time_limit_ms);
                zhandi.try_move(ai_step);
            }

            if ui.button("ai play").clicked() {
                let ai = MCTSv2::new(evaluate, quick_move);
                let ai_step = ai.play(zhandi.board.clone(), ai_time_limit_ms);
                zhandi.try_move(ai_step);
            }
        });

    Ok(())
}