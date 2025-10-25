use std::marker::PhantomData;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{
    general::{board::*, game::Game as GameTrait}, net::NetState, tree::{game_tree::GameTree, game_tree_event::MoveToNodeEvent}, ui::ui_menu::UiMenuState
};

#[derive(Resource)]
pub struct UiSlState<G: GameTrait> {
    load_fen: String,
    load_fen_error: String,
    load_pgn: String,
    load_pgn_error: String,
    load_tree: String, 
    load_tree_error: String,
    _marker: PhantomData<G>,
}

impl<G: GameTrait> Default for UiSlState<G> {
    fn default() -> Self {
        Self { 
            load_fen: Default::default(), 
            load_fen_error: Default::default(), 
            load_pgn: Default::default(), 
            load_pgn_error: Default::default(), 
            load_tree: Default::default(), 
            load_tree_error: Default::default(), 
            _marker: PhantomData,
        }
    }
}

pub fn ui_sl<G: GameTrait>(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    mut ui_sl: ResMut<UiSlState::<G>>,
    mut q_game: Query<&mut G>,
    mut ew_mtn: EventWriter<MoveToNodeEvent>,
    net_state: Res<NetState>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let Ok(mut game) = q_game.single_mut() else {
        return Ok(())
    };

    let disconnected = match *net_state {
        NetState::Disconnected => true,
        NetState::Listening(_, _) => false,
        NetState::Connected(_, _) => false,
    };

    egui::Window::new("Save & Load")
        .open(&mut ui_menu.sl_window_open)
        .show(ctx, |ui| {
            if disconnected {
                if ui.button("New Game").clicked() {
                    let tree = GameTree::<G::B>::new(G::B::default());
                    *game.tree() = tree;
                    game.tree().move_to_start(&mut ew_mtn);
                }

                ui.separator();
            }

            if ui.button("Copy current FEN").clicked() {
                ctx.copy_text(game.board().write_fen());
            }

            if ui.button("Copy current game tree").clicked() {
                ctx.copy_text(game.tree().to_string());
            }

            if disconnected {
                ui.horizontal(|ui| {
                    ui.label("Load FEN: ");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(), // Use available_size to let TextEdit fill the scroll area
                            egui::TextEdit::multiline(&mut ui_sl.load_fen)
                                .desired_width(f32::INFINITY) // Ensure it takes full width
                                .code_editor(), // Optional: for code editor features
                        );
                    });
                });
                ui.horizontal(|ui| {
                    if ui.button("Load").clicked() {
                        if let Some(board) = G::B::read_fen(ui_sl.load_fen.clone()) {
                            let tree = GameTree::<G::B>::new(board);
                            *game.tree() = tree;
                            game.tree().move_to_start(&mut ew_mtn);
                            ui_sl.load_fen_error = String::new();
                        } else {
                            ui_sl.load_fen_error = "invalid FEN".to_string();
                        }
                    }
                    ui.label(ui_sl.load_fen_error.clone());
                });

                ui.horizontal(|ui| {
                    ui.label("Load game tree: ");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(), // Use available_size to let TextEdit fill the scroll area
                            egui::TextEdit::multiline(&mut ui_sl.load_tree)
                                .desired_width(f32::INFINITY) // Ensure it takes full width
                                .code_editor(), // Optional: for code editor features
                        );
                    });
                });
                ui.horizontal(|ui| {
                    if ui.button("Load").clicked() {
                        if let Some(tree) = GameTree::<G::B>::from_string(ui_sl.load_tree.clone()) {
                            *game.tree() = tree;
                            game.tree().move_to_start(&mut ew_mtn);
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
                    if let Some(tree) = GameTree::<G::B>::from_pgn(ui_sl.load_pgn.clone()) {
                        *game.tree() = tree;
                        game.tree().move_to_start(&mut ew_mtn);
                        ui_sl.load_pgn_error = String::new();
                    } else {
                        ui_sl.load_pgn_error = "invalid PGN".to_string();
                    }
                }
            }
        });

    Ok(())
}