use bevy::prelude::*;
use bevy_egui::{egui::{self}, EguiContexts};

use crate::{ai::{mcts::MCTSAI, mctsv2::MCTSv2, AI}, general::Board, xingxiang::{game::XingxiangGame, ai::*}, ui::ui_menu::UiMenuState};

pub fn ui_xingxiang(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    mut q_xingxiang: Query<&mut XingxiangGame>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let Ok(mut xingxiang) = q_xingxiang.single_mut() else {
        return Ok(())
    };

    let ai_time_limit_ms = ui_menu.ai_time_limit_ms;

    egui::Window::new("Xingxiang")
        .open(&mut ui_menu.xingxiang_window_open)
        .show(ctx, |ui| {
            ui.label(xingxiang.board.game_info());

            ui.label(format!("Now is turn {}", xingxiang.board.get_fullmove()));

            if ui.button("weak ai play").clicked() {
                let ai = MCTSAI::new();
                let ai_step = ai.play(xingxiang.board.clone(), ai_time_limit_ms);
                xingxiang.try_move(ai_step);
            }

            if ui.button("ai play").clicked() {
                let ai = MCTSv2::new(evaluate, quick_move);
                let ai_step = ai.play(xingxiang.board.clone(), ai_time_limit_ms);
                xingxiang.try_move(ai_step);
            }
        });

    Ok(())
}