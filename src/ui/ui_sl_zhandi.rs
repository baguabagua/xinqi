use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{
    general::board::*, zhandi::{game::ZhandiGame, general::ZhandiBoard}, tree::{game_tree::GameTree, game_tree_event::MoveToNodeEvent}, ui::ui_menu::UiMenuState
};

#[derive(Default, Resource)]
pub struct UiSlStateZhandi {
    load_fen: String,
    load_fen_error: String,
    load_pgn: String,
    load_pgn_error: String,
    load_tree: String, 
    load_tree_error: String,
}

pub fn ui_sl_zhandi(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    mut ui_sl: ResMut<UiSlStateZhandi>,
    mut q_zhandi: Query<&mut ZhandiGame>,
    mut ew_mtn: EventWriter<MoveToNodeEvent>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let Ok(mut zhandi) = q_zhandi.single_mut() else {
        return Ok(())
    };

    egui::Window::new("Save & Load")
        .open(&mut ui_menu.sl_window_open)
        .show(ctx, |ui| {
            if ui.button("New Game").clicked() {
                let tree = GameTree::<ZhandiBoard>::new(ZhandiBoard::default());
                zhandi.tree = tree;
                zhandi.tree.move_to_start(&mut ew_mtn);
            }

            ui.separator();

            if ui.button("Copy current FEN").clicked() {
                ctx.copy_text(zhandi.board.write_fen());
            }

            if ui.button("Copy current game tree").clicked() {
                ctx.copy_text(zhandi.tree.to_string());
            }

            ui.horizontal(|ui| {
                ui.label("Load FEN: ");
                egui::TextEdit::multiline(&mut ui_sl.load_fen)
                    .desired_rows(4)
                    .show(ui);
            });
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    if let Some(board) = ZhandiBoard::read_fen(ui_sl.load_fen.clone()) {
                        let tree = GameTree::<ZhandiBoard>::new(board);
                        zhandi.tree = tree;
                        zhandi.tree.move_to_start(&mut ew_mtn);
                        ui_sl.load_fen_error = String::new();
                    } else {
                        ui_sl.load_fen_error = "invalid FEN".to_string();
                    }
                }
                ui.label(ui_sl.load_fen_error.clone());
            });

            ui.horizontal(|ui| {
                ui.label("Load game tree: ");
                egui::TextEdit::multiline(&mut ui_sl.load_tree)
                    .desired_rows(4)
                    .show(ui);
            });
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    if let Some(tree) = GameTree::<ZhandiBoard>::from_string(ui_sl.load_tree.clone()) {
                        zhandi.tree = tree;
                        zhandi.tree.move_to_start(&mut ew_mtn);
                        ui_sl.load_tree_error = String::new();
                    } else {
                        ui_sl.load_tree_error = "invalid tree text".to_string();
                    }
                }
                ui.label(ui_sl.load_tree_error.clone());
            });

            ui.horizontal(|ui| {
                ui.label("Load PGN: ");
                ui.text_edit_singleline(&mut ui_sl.load_pgn);
            });
            if ui.button("Load").clicked() {
                if let Some(tree) = GameTree::<ZhandiBoard>::from_pgn(ui_sl.load_pgn.clone()) {
                    zhandi.tree = tree;
                    zhandi.tree.move_to_start(&mut ew_mtn);
                    ui_sl.load_pgn_error = String::new();
                } else {
                    ui_sl.load_pgn_error = "invalid PGN".to_string();
                }
            }
        });

    Ok(())
}