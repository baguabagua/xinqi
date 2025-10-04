use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{hequn::{game::{EndHequnGame, HequnGame}}};

#[derive(Resource)]
pub struct UiMenuState {
    game: GameTitle,
    pub running_game: Option<Game>,
    pub sl_window_open: bool,
    pub tree_window_open: bool,
    pub hequn_window_open: bool,
}

impl Default for UiMenuState {
    fn default() -> Self {
        Self { 
            game: GameTitle::Hequn, 
            running_game: None,
            sl_window_open: false, 
            tree_window_open: false, 
            hequn_window_open: false,
        }
    }
}

#[derive(PartialEq, Debug)]
enum GameTitle {
    Hequn,
}

pub enum Game {
    Hequn(Entity),
}

pub fn ui_menu(
    mut ui_state: ResMut<UiMenuState>,
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut ew_end_hequn: EventWriter<EndHequnGame>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Menu");

            ui.label(format!("Now playing: {}", match ui_state.running_game {
                Some(Game::Hequn(_)) => "Hequn",
                None => "None",
            }));

            ui.separator();

            let UiMenuState {
                game,
                running_game,
                sl_window_open,
                tree_window_open,
                hequn_window_open,
            } = &mut *ui_state;

            egui::ComboBox::from_label("Choose a game")
                .selected_text(format!("{:?}", game)) // use debug trait
                .show_ui(ui, |ui| {
                    ui.selectable_value(game, GameTitle::Hequn, "Hequn");
                });

            // 这种实现方式有可能导致同一帧存在两个游戏实体，尽管它们不会在同一帧被绘制
            if ui.button("Start New Game").clicked() {
                match running_game {
                    Some(old_game) => {
                        match old_game {
                            Game::Hequn(hequn) => {
                                ew_end_hequn.write(EndHequnGame { game_entity: *hequn });
                            },
                        }
                    },
                    None => {},
                }
                match game {
                    GameTitle::Hequn => {
                        *running_game = Some(Game::Hequn(commands.spawn((
                            HequnGame::new(),
                        )).id()));
                    },
                }
            }

            ui.separator();

            ui.checkbox(sl_window_open, "show SL window");
            ui.checkbox(tree_window_open, "show game tree");
            ui.checkbox(hequn_window_open, "show hequn game");
        });

    Ok(())
}