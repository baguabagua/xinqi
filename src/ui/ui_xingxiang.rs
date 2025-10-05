use bevy::prelude::*;
use bevy_egui::{egui::{self}, EguiContexts};

use crate::{general::Board, xingxiang::{game::XingxiangGame}, ui::ui_menu::UiMenuState};

pub fn ui_xingxiang(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    q_xingxiang: Query<&XingxiangGame>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let Ok(xingxiang) = q_xingxiang.single() else {
        return Ok(())
    };

    egui::Window::new("Xingxiang")
        .open(&mut ui_menu.xingxiang_window_open)
        .show(ctx, |ui| {
            ui.label(xingxiang.board.game_info());

            ui.label(format!("Now is turn {}", xingxiang.board.get_fullmove()))
        });

    Ok(())
}