use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{
    general::board::*, xingxiang::{game::XingxiangGame, general::XingxiangBoard}, tree::{game_tree::GameTree, game_tree_event::MoveToNodeEvent}, ui::ui_menu::UiMenuState
};

#[derive(Default, Resource)]
pub struct UiSlStateXingxiang {
    load_fen: String,
    load_fen_error: String,
    load_pgn: String,
    load_pgn_error: String,
    load_tree: String, 
    load_tree_error: String,
}

pub fn ui_sl_xingxiang(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    mut ui_sl: ResMut<UiSlStateXingxiang>,
    mut q_xingxiang: Query<&mut XingxiangGame>,
    mut ew_mtn: EventWriter<MoveToNodeEvent>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let Ok(mut xingxiang) = q_xingxiang.single_mut() else {
        return Ok(())
    };

    egui::Window::new("Save & Load")
        .open(&mut ui_menu.sl_window_open)
        .show(ctx, |ui| {
            if ui.button("New Game").clicked() {
                let tree = GameTree::<XingxiangBoard>::new(XingxiangBoard::default());
                xingxiang.tree = tree;
                xingxiang.tree.move_to_start(&mut ew_mtn);
            }

            ui.separator();

            if ui.button("Copy current FEN").clicked() {
                ctx.copy_text(xingxiang.board.write_fen());
            }

            if ui.button("Copy current game tree").clicked() {
                ctx.copy_text(xingxiang.tree.to_string());
            }

            ui.horizontal(|ui| {
                ui.label("Load FEN: ");
                egui::TextEdit::multiline(&mut ui_sl.load_fen)
                    .desired_rows(4)
                    .show(ui);
            });
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    if let Some(board) = XingxiangBoard::read_fen(ui_sl.load_fen.clone()) {
                        let tree = GameTree::<XingxiangBoard>::new(board);
                        xingxiang.tree = tree;
                        xingxiang.tree.move_to_start(&mut ew_mtn);
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
                    if let Some(tree) = GameTree::<XingxiangBoard>::from_string(ui_sl.load_tree.clone()) {
                        xingxiang.tree = tree;
                        xingxiang.tree.move_to_start(&mut ew_mtn);
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
                if let Some(tree) = GameTree::<XingxiangBoard>::from_pgn(ui_sl.load_pgn.clone()) {
                    xingxiang.tree = tree;
                    xingxiang.tree.move_to_start(&mut ew_mtn);
                    ui_sl.load_pgn_error = String::new();
                } else {
                    ui_sl.load_pgn_error = "invalid PGN".to_string();
                }
            }
        });

    Ok(())
}