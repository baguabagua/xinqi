use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    general::PlayerOrder, hequn::game::{EndHequnGame, HequnGame}, net::{message::{Message, ReceiveRemoteStep, SendRemoteStep}, NetCommand, NetState, ReceiveNetMsgEvent, SendNetMsgEvent}, xingxiang::game::{EndXingxiangGame, XingxiangGame}, zhandi::game::{EndZhandiGame, ZhandiGame}
};

struct GameRequest {
    game_name: String, 
    player_order: bool, 
}

#[derive(Resource)]
pub struct UiMenuState {
    game: GameTitle,
    order: PlayerOrder,
    pub running_game: Option<Game>,
    pub sl_window_open: bool,
    pub tree_window_open: bool,
    pub hequn_window_open: bool,
    pub zhandi_window_open: bool,
    pub xingxiang_window_open: bool,
    pub ai_time_limit_ms: u32,
    
    local_addr: String,
    remote_addr: String,
    game_request: Option<GameRequest>, // 暂存对方发来的开始对局请求
}

impl Default for UiMenuState {
    fn default() -> Self {
        Self { 
            game: GameTitle::Hequn,
            order: PlayerOrder::First, 
            running_game: None,
            sl_window_open: false, 
            tree_window_open: false, 
            hequn_window_open: false,
            zhandi_window_open: false,
            xingxiang_window_open: false,
            ai_time_limit_ms: 2000,
            local_addr: String::from("0.0.0.0:18386"),
            remote_addr: String::from("123.123.123.123:18386"),
            game_request: None,
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
    net_state: Res<NetState>,
    mut ew_nc: EventWriter<NetCommand>,
    mut er_net: EventReader<ReceiveNetMsgEvent>,
    mut ew_net: EventWriter<SendNetMsgEvent>,
    mut er_step: EventReader<SendRemoteStep>,
    mut ew_step: EventWriter<ReceiveRemoteStep>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Menu");

            let UiMenuState {
                game,
                order,
                running_game,
                sl_window_open,
                tree_window_open,
                hequn_window_open,
                zhandi_window_open,
                xingxiang_window_open,
                ai_time_limit_ms,
                local_addr,
                remote_addr,
                game_request,
            } = &mut *ui_state;

            let mut disconnected = false;
            let mut connected = false;

            match *net_state {
                NetState::Disconnected => {
                    disconnected = true;
                    ui.label("Net State: Disconnected");
                    ui.horizontal(|ui| {
                        ui.label("Listen to: ");
                        ui.text_edit_singleline(local_addr);
                    });
                    if ui.button("Listen").clicked() {
                        ew_nc.write(NetCommand::Listen(local_addr.clone()));
                    }
                    ui.horizontal(|ui| {
                        ui.label("Connect to: ");
                        ui.text_edit_singleline(remote_addr);
                    });
                    if ui.button("Connect").clicked() {
                        ew_nc.write(NetCommand::Connect(remote_addr.clone()));
                    }
                    *game_request = None;
                },
                NetState::Listening(addr, _) => {
                    ui.label(format!("Net State: Listening to {}", addr));
                    if ui.button("Disconnect").clicked() {
                        ew_nc.write(NetCommand::Disconnect);
                    }
                    *game_request = None;
                },
                NetState::Connected(addr, _) => {
                    connected = true;
                    ui.label(format!("Net State: Connected to {}", addr));
                    if ui.button("Disconnect").clicked() {
                        ew_nc.write(NetCommand::Disconnect);
                    }
                },
            }

            ui.separator();

            if disconnected {

                ui.label(format!("Now playing: {}", match running_game {
                    Some(Game::Hequn(_)) => "Hequn",
                    Some(Game::Zhandi(_)) => "Zhandi",
                    Some(Game::Xingxiang(_)) => "Xingxiang",
                    None => "None",
                }));

                ui.separator();

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
                                HequnGame::new(None),
                            )).id()));
                        },
                        GameTitle::Zhandi => {
                            *running_game = Some(Game::Zhandi(commands.spawn((
                                ZhandiGame::new(None),
                            )).id()));
                        },
                        GameTitle::Xingxiang => {
                            *running_game = Some(Game::Xingxiang(commands.spawn((
                                XingxiangGame::new(None),
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

            } // if disconnected

            if connected {
                *tree_window_open = false;
                *hequn_window_open = false;
                *zhandi_window_open = false;
                *xingxiang_window_open = false;

                for event in er_net.read() {
                    info!("receive message from net: {}", event.message);
                    let Ok(message) = serde_json::from_str(&event.message) else { continue; };
                    match message {
                        Message::CreateNewGame { game_name, player_order } => {
                            *game_request = Some(GameRequest {
                                game_name,
                                player_order,
                            });
                        },
                        Message::AcceptCreateNewGame { game_name, player_order } => {
                            let remote_play = Some(if player_order { PlayerOrder::First } else { PlayerOrder::Second });
                            start_game(running_game, &game_name, remote_play, &mut commands, &mut ew_end_hequn, &mut ew_end_zhandi, &mut ew_end_xingxiang);
                        },
                        Message::Step(step) => {
                            ew_step.write(ReceiveRemoteStep { step });
                        },
                    }
                }

                for event in er_step.read() {
                    // info!("send step: {}", event.step);
                    let message = Message::Step(event.step.clone());
                    let message = serde_json::to_string(&message).unwrap();
                    ew_net.write(SendNetMsgEvent { message });
                }

                egui::ComboBox::from_label("Choose a game")
                    .selected_text(format!("{:?}", game)) // use debug trait
                    .show_ui(ui, |ui| {
                        ui.selectable_value(game, GameTitle::Hequn, "Hequn");
                        ui.selectable_value(game, GameTitle::Zhandi, "Zhandi");
                        ui.selectable_value(game, GameTitle::Xingxiang, "Xingxiang");
                    });

                egui::ComboBox::from_label("Choose player order")
                    .selected_text(format!("{:?}", order)) // use debug trait 
                    .show_ui(ui, |ui| {
                        ui.selectable_value(order, PlayerOrder::First, "First");
                        ui.selectable_value(order, PlayerOrder::Second, "Second");
                    });

                if ui.button("Send Remote Game Invitation").clicked() {
                    let message = Message::CreateNewGame { 
                        game_name: format!("{:?}", game), 
                        player_order: *order == PlayerOrder::First,
                    };
                    let message = serde_json::to_string(&message).unwrap();
                    ew_net.write(SendNetMsgEvent { message });
                }

                if let Some(request) = game_request {
                    ui.label(format!("Receive Remote Game Invitation: {}", request.game_name));
                    ui.label(format!("Your Order: {}", if request.player_order { "Second" } else { "First" }));
                    if ui.button("Accept").clicked() {
                        let message = Message::AcceptCreateNewGame { 
                            game_name: request.game_name.clone(), 
                            player_order: !request.player_order, 
                        };
                        let message = serde_json::to_string(&message).unwrap();
                        ew_net.write(SendNetMsgEvent { message });
                        let remote_play = Some(if request.player_order { PlayerOrder::First } else { PlayerOrder::Second });
                        start_game(running_game, &request.game_name, remote_play, &mut commands, &mut ew_end_hequn, &mut ew_end_zhandi, &mut ew_end_xingxiang);
                    }
                }

                ui.checkbox(sl_window_open, "show SL window");
            } // if connected
        });

    Ok(())
}

fn start_game(
    running_game: &mut Option<Game>,
    game_name: &String,
    remote_play: Option<PlayerOrder>,
    commands: &mut Commands,
    ew_end_hequn: &mut EventWriter<EndHequnGame>,
    ew_end_zhandi: &mut EventWriter<EndZhandiGame>,
    ew_end_xingxiang: &mut EventWriter<EndXingxiangGame>,
) {
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
    match game_name.as_str() {
        "Hequn" => {
            *running_game = Some(Game::Hequn(commands.spawn((
                HequnGame::new(remote_play),
            )).id()));
        },
        "Zhandi" => {
            *running_game = Some(Game::Zhandi(commands.spawn((
                ZhandiGame::new(remote_play),
            )).id()));
        },
        "Xingxiang" => {
            *running_game = Some(Game::Xingxiang(commands.spawn((
                XingxiangGame::new(remote_play),
            )).id()));
        },
        _ => { *running_game = None; },
    }
}