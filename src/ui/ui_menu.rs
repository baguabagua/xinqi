use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    hequn::game::{EndHequnGame, HequnGame}, 
    zhandi::game::{EndZhandiGame, ZhandiGame},
    xingxiang::game::{EndXingxiangGame, XingxiangGame},
};

#[derive(Resource)]
pub struct UiMenuState {
    game: GameTitle,
    pub running_game: Option<Game>,
    pub sl_window_open: bool,
    pub tree_window_open: bool,
    pub hequn_window_open: bool,
    pub zhandi_window_open: bool,
    pub xingxiang_window_open: bool,
    pub ai_time_limit_ms: u32,
}

impl Default for UiMenuState {
    fn default() -> Self {
        Self { 
            game: GameTitle::Hequn, 
            running_game: None,
            sl_window_open: false, 
            tree_window_open: false, 
            hequn_window_open: false,
            zhandi_window_open: false,
            xingxiang_window_open: false,
            ai_time_limit_ms: 2000,
        }
    }
}

#[derive(PartialEq, Debug)]
enum GameTitle {
    Hequn,
    Zhandi,
    Xingxiang,
}

pub enum Game {
    Hequn(Entity),
    Zhandi(Entity),
    Xingxiang(Entity),
}

pub fn ui_menu(
    mut ui_state: ResMut<UiMenuState>,
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut ew_end_hequn: EventWriter<EndHequnGame>,
    mut ew_end_zhandi: EventWriter<EndZhandiGame>,
    mut ew_end_xingxiang: EventWriter<EndXingxiangGame>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Menu");

            ui.label(format!("Now playing: {}", match ui_state.running_game {
                Some(Game::Hequn(_)) => "Hequn",
                Some(Game::Zhandi(_)) => "Zhandi",
                Some(Game::Xingxiang(_)) => "Xingxiang",
                None => "None",
            }));

            ui.separator();

            let UiMenuState {
                game,
                running_game,
                sl_window_open,
                tree_window_open,
                hequn_window_open,
                zhandi_window_open,
                xingxiang_window_open,
                ai_time_limit_ms,
            } = &mut *ui_state;

            egui::ComboBox::from_label("Choose a game")
                .selected_text(format!("{:?}", game)) // use debug trait
                .show_ui(ui, |ui| {
                    ui.selectable_value(game, GameTitle::Hequn, "Hequn");
                    ui.selectable_value(game, GameTitle::Zhandi, "Zhandi");
                    ui.selectable_value(game, GameTitle::Xingxiang, "Xingxiang");
                });

            // 这种实现方式有可能导致同一帧存在两个游戏实体，尽管它们不会在同一帧被绘制
            if ui.button("Start New Game").clicked() {
                *sl_window_open = false;
                *tree_window_open = false;
                *hequn_window_open = false;
                *zhandi_window_open = false;
                *xingxiang_window_open = false;
                match running_game {
                    Some(old_game) => {
                        match old_game {
                            Game::Hequn(hequn) => {
                                ew_end_hequn.write(EndHequnGame { game_entity: *hequn });
                            },
                            Game::Zhandi(zhandi) => {
                                ew_end_zhandi.write(EndZhandiGame { game_entity: *zhandi });
                            },
                            Game::Xingxiang(xingxiang) => {
                                ew_end_xingxiang.write(EndXingxiangGame { game_entity: *xingxiang });
                            }
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
                    GameTitle::Zhandi => {
                        *running_game = Some(Game::Zhandi(commands.spawn((
                            ZhandiGame::new(),
                        )).id()));
                    },
                    GameTitle::Xingxiang => {
                        *running_game = Some(Game::Xingxiang(commands.spawn((
                            XingxiangGame::new(),
                        )).id()));
                    },
                }
            }

            ui.separator();

            ui.checkbox(sl_window_open, "show SL window");
            ui.checkbox(tree_window_open, "show game tree");
            match running_game {
                Some(Game::Hequn(_)) => {
                    ui.checkbox(hequn_window_open, "show hequn game");
                },
                Some(Game::Zhandi(_)) => {
                    ui.checkbox(zhandi_window_open, "show zhandi game");
                },
                Some(Game::Xingxiang(_)) => {
                    ui.checkbox(xingxiang_window_open, "show xingxiang game");
                },
                None => {},
            }

            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Set AI time limit:");
                ui.add(egui::DragValue::new(ai_time_limit_ms));
                ui.label("ms");
            });
        });

    Ok(())
}