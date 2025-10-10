use bevy::prelude::*;
use bevy_egui::{egui::{self, Ui, Grid}, EguiContexts};

use crate::{general::game::Game as GameTrait, tree::game_tree_event::{DeleteVariationEvent, MoveToNodeEvent}, ui::ui_menu::*};

pub fn ui_game_tree<G>(
    mut contexts: EguiContexts,
    mut ui_menu: ResMut<UiMenuState>,
    mut q_game: Query<&mut G>,
    mut ew_mtn: EventWriter<MoveToNodeEvent>,
    mut ew_dv: EventWriter<DeleteVariationEvent>,
) -> Result 
where 
    G: GameTrait,
{
    let ctx = contexts.ctx_mut()?;

    let Ok(mut game) = q_game.single_mut() else {
        return Ok(())
    };

    egui::Window::new("Game Tree")
        .open(&mut ui_menu.tree_window_open)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    game.tree().display_egui(ui, &mut ew_mtn, &mut ew_dv);
                });

            ui.separator();

            egui::TopBottomPanel::bottom("button_panel")
                .show_inside(ui, |ui| {
                    let total_width = ui.available_width();
                    let btn_width = total_width * 0.20;
                    let is_first_board = game.tree().is_first_board();
                    let is_last_board = game.tree().is_last_board();

                    Grid::new("navigation_buttons")
                        .num_columns(4)
                        .min_col_width(btn_width)
                        .show(ui, |ui| {
                            let response = ui.add_enabled(!is_first_board, |ui: &mut Ui| {
                                ui.add_sized([btn_width, 0.0], egui::Button::new("<<"))
                            });
                            if response.clicked() {
                                game.tree().move_to_start(&mut ew_mtn);
                            }

                            let response = ui.add_enabled(!is_first_board, |ui: &mut Ui| {
                                ui.add_sized([btn_width, 0.0], egui::Button::new("<"))
                            });
                            if response.clicked() {
                                game.tree().move_backward(&mut ew_mtn);
                            }
                            
                            let response = ui.add_enabled(!is_last_board, |ui: &mut Ui| {
                                ui.add_sized([btn_width, 0.0], egui::Button::new(">"))
                            });
                            if response.clicked() {
                                game.tree().move_forward(&mut ew_mtn);
                            }

                            let response = ui.add_enabled(!is_last_board, |ui: &mut Ui| {
                                ui.add_sized([btn_width, 0.0], egui::Button::new(">>"))
                            });
                            if response.clicked() {
                                game.tree().move_to_end(&mut ew_mtn);
                            }
                        });
                });
        });

    Ok(())
}