use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};

use crate::{general::*, graphics::{entity::{CellCom, Shape}, interaction::{ClickEvent, DragEvent}}, hequn::{general::{HequnBoard, HequnStep}, utils::*}, net::message::{ReceiveRemoteStep, SendRemoteStep}, tree::game_tree::GameTree};

#[derive(Component)]
pub struct HequnGame {
    pub board: HequnBoard,
    pub tree: GameTree<HequnBoard>,
    rect: Rect,
    cells: Vec<Vec<Entity>>,
    pieces: Vec<Vec<Entity>>,
    background: Entity,
    updated: bool,
    
    remote_play: Option<PlayerOrder>, 
}

impl HequnGame {
    pub fn new(remote_play: Option<PlayerOrder>) -> Self {
        Self {
            board: HequnBoard::default(),
            tree: GameTree::new(HequnBoard::default()),
            rect: Rect::from_center_size(Vec2::ZERO, Vec2::new(600.0, 600.0)),
            cells: vec![vec![Entity::PLACEHOLDER; BOARD_SIZE_J]; BOARD_SIZE_I],
            pieces: vec![vec![Entity::PLACEHOLDER; BOARD_SIZE_J]; BOARD_SIZE_I],
            background: Entity::PLACEHOLDER,
            updated: false,
            remote_play,
        }
    }
    pub fn try_move(&mut self, step: HequnStep) {
        if self.tree.try_move(step) {
            self.updated = false;
            self.board = self.tree.board();
        }
    }
}

impl Game for HequnGame {
    type B = HequnBoard;
    
    fn tree(&mut self) -> &mut GameTree<Self::B> {
        &mut self.tree
    }
    
    fn board(&self) -> &Self::B {
        &self.board
    }
}

#[derive(Resource)]
pub struct HequnTextureAssets {
    background: Handle<Image>,
    grey_grid: Handle<Image>,
    black_grid: Handle<Image>,
    white_grid: Handle<Image>,
    black_piece: Handle<Image>,
    white_piece: Handle<Image>,
}

// 绘制纹理储存在 HequnTextureAssets 中
pub fn hequn_setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let mut hequn_background = Image::new_fill(
        Extent3d { width: 600, height: 600, depth_or_array_layers: 1 }, 
        TextureDimension::D2, 
        &[255, 255, 255, 255], 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::all(), 
    );
    for x in 49..551 {
        for y in 49..551 {
            let _ = hequn_background.set_color_at(x, y, Color::BLACK);
        }
    }

    let hequn_grey_grid = Image::new_fill(
        Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, 
        TextureDimension::D2, 
        &[160, 160, 160, 255], 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::all(), 
    );
    let hequn_black_grid = Image::new_fill(
        Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, 
        TextureDimension::D2, 
        &[249, 106, 226, 255], 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::all(), 
    );
    let hequn_white_grid = Image::new_fill(
        Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, 
        TextureDimension::D2, 
        &[127, 246, 244, 255], 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::all(), 
    );
    let hequn_black_piece = {
        let size = 50;
        let radius = 20.0;
        let center = (size as f32 / 2.0, size as f32 / 2.0);

        let mut pixels = Vec::with_capacity(size * size * 4);

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center.0;
                let dy = y as f32 - center.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    // 圆形内部：黑色 (0, 0, 0, 255)
                    pixels.extend_from_slice(&[0, 0, 0, 255]);
                } else {
                    // 圆形外部：完全透明 (0, 0, 0, 0)
                    pixels.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }

        Image::new(
            Extent3d {
                width: size as u32,
                height: size as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(), 
        )
    };
    let hequn_white_piece = {
        let size = 50;
        let radius = 20.0;
        let border_width = 2.0;
        let center = (size as f32 / 2.0, size as f32 / 2.0);

        let mut pixels = Vec::with_capacity(size * size * 4);

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center.0;
                let dy = y as f32 - center.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius - border_width {
                    // 圆形内部（去掉边框区域）：白色 (255, 255, 255, 255)
                    pixels.extend_from_slice(&[255, 255, 255, 255]);
                } else if distance <= radius {
                    // 边框区域：黑色 (0, 0, 0, 255)
                    pixels.extend_from_slice(&[0, 0, 0, 255]);
                } else {
                    // 圆形外部：完全透明 (0, 0, 0, 0)
                    pixels.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }

        Image::new(
            Extent3d {
                width: size as u32,
                height: size as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(), 
        )
    };

    let textures = HequnTextureAssets {
        background: images.add(hequn_background),
        grey_grid: images.add(hequn_grey_grid),
        black_grid: images.add(hequn_black_grid),
        white_grid: images.add(hequn_white_grid),
        black_piece: images.add(hequn_black_piece),
        white_piece: images.add(hequn_white_piece),
    };
    commands.insert_resource(textures);
}

fn draw(
    commands: &mut Commands,
    textures: &HequnTextureAssets,
    game: &mut HequnGame,
) {
    game.background = commands.spawn((
        {
            let mut sprite = Sprite::from_image(textures.background.clone());
            sprite.custom_size = Some(game.rect.size());
            sprite
        },
        {
            Transform::from_translation(game.rect.center().extend(0.0))
        }
    )).id();

    let leftdown = game.rect.center() - game.rect.size() * 0.375;
    let cell_size = game.rect.size() / 12.0;
    let (dx, dy) = (cell_size.x, cell_size.y);
    let dcell_size = cell_size - Vec2::new(2.0, 2.0);
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            let piece = if let Some(p) = game.board.pieces[x][y] {
                commands.spawn((
                    {
                        let mut sprite = Sprite::from_image(match p {
                            super::general::HequnPiece::Black => textures.black_piece.clone(),
                            super::general::HequnPiece::White => textures.white_piece.clone(),
                        });
                        sprite.custom_size = Some(dcell_size);
                        sprite
                    },
                    {
                        Transform::from_translation((leftdown + Vec2::new(x as f32 * dx, y as f32 * dy)).extend(2.0))
                    }
                )).id()
            } else {
                Entity::PLACEHOLDER
            };
            game.pieces[x][y] = piece;

            let clickable = match game.remote_play {
                Some(remote_player) => remote_player != game.board.get_active_player(),
                None => true,
            };
            let cell = commands.spawn((
                CellCom {
                    shape: Shape::Rect { rect: Rect::from_center_size(leftdown + Vec2::new(x as f32 * dx, y as f32 * dy), dcell_size) },
                    clickable,
                    dragable: false,
                    upper_piece: Entity::PLACEHOLDER,
                },
                {
                    let mut sprite = Sprite::from_image(match game.board.cells[x][y] {
                        super::general::HequnCell::Grey => textures.grey_grid.clone(),
                        super::general::HequnCell::Colored(hequn_piece) => match hequn_piece {
                            super::general::HequnPiece::Black => textures.black_grid.clone(),
                            super::general::HequnPiece::White => textures.white_grid.clone(),
                        },
                    });
                    sprite.custom_size = Some(dcell_size);
                    sprite
                },
                {
                    Transform::from_translation((leftdown + Vec2::new(x as f32 * dx, y as f32 * dy)).extend(1.0))
                }
            )).id();
            game.cells[x][y] = cell;
        }
    }
}

fn clear(
    commands: &mut Commands,
    game: &mut HequnGame,
) {
    if game.background != Entity::PLACEHOLDER {
        commands.entity(game.background).despawn();
    }
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            if game.pieces[x][y] != Entity::PLACEHOLDER {
                commands.entity(game.pieces[x][y]).despawn();
            }
            if game.cells[x][y] != Entity::PLACEHOLDER {
                commands.entity(game.cells[x][y]).despawn();
            }
        }
    }
}

fn update(
    commands: &mut Commands,
    game: &mut HequnGame,
    _er_drag: &mut EventReader<DragEvent>,
    er_click: &mut EventReader<ClickEvent>,
    er_update: &mut EventReader<UpdateBoard<HequnBoard>>,
    er_remote: &mut EventReader<ReceiveRemoteStep>,
    ew_remote: &mut EventWriter<SendRemoteStep>,
    textures: &HequnTextureAssets,
) {
    match game.remote_play {
        Some(remote_player) => {
            for event in er_click.read() {
                if game.board.get_active_player() == remote_player {
                    break;
                }
                for x in 0..BOARD_SIZE_I {
                    for y in 0..BOARD_SIZE_J {
                        if game.cells[x][y] == event.cell {
                            let step = HequnStep::Pos(x, y);
                            if let Some(step_str) = game.board.write_step(step) {
                                game.try_move(step);
                                // info!("hequn send step: {}", step_str);
                                ew_remote.write(SendRemoteStep { step: step_str });
                            }
                        }
                    }
                }
            }

            for event in er_remote.read() {
                // info!("hequn receive step: {}", event.step);
                if game.board.get_active_player() == remote_player.flip() {
                    break;
                }
                //info!("player correct");
                if let Some(step) = game.board.read_step(event.step.clone()) {
                    // info!("step valid");
                    game.try_move(step);
                }
            }
        },
        None => {
            for event in er_click.read() {
                for x in 0..BOARD_SIZE_I {
                    for y in 0..BOARD_SIZE_J {
                        if game.cells[x][y] == event.cell {
                            game.try_move(HequnStep::Pos(x, y));
                        }
                    }
                }
            }
        },
    }

    for event in er_update.read() {
        game.updated = false;
        game.board = event.new_board.clone();
    }

    if !game.updated {
        clear(commands, game);
        draw(commands, textures, game);
        game.updated = true;
    }
}

pub fn hequn_update(
    mut commands: Commands,
    mut q_game: Query<&mut HequnGame>,
    mut er_drag: EventReader<DragEvent>,    
    mut er_click: EventReader<ClickEvent>,
    mut er_update: EventReader<UpdateBoard<HequnBoard>>,
    mut er_remote: EventReader<ReceiveRemoteStep>,
    mut ew_remote: EventWriter<SendRemoteStep>,
    textures: Res<HequnTextureAssets>,
) {
    for mut game in q_game.iter_mut() {
        update(&mut commands, &mut game, &mut er_drag, &mut er_click, &mut er_update, &mut er_remote, &mut ew_remote, &textures);
    }
}

// 以下代码由 Deepseek 生成

#[derive(Event)]
pub struct EndHequnGame {
    pub game_entity: Entity,
}

// 处理游戏结束的系统
pub fn handle_end_hequn_game(
    mut commands: Commands,
    mut end_events: EventReader<EndHequnGame>,
    q_game: Query<&mut HequnGame>,
) {
    for event in end_events.read() {
        if let Ok(game) = q_game.get(event.game_entity) {
            // 先清理所有子实体
            cleanup_game_entities(&mut commands, game);
            
            // 然后删除游戏实体本身
            commands.entity(event.game_entity).despawn();
        }
    }
}

// 清理游戏所有子实体的辅助函数
fn cleanup_game_entities(commands: &mut Commands, game: &HequnGame) {
    // 清理背景
    if game.background != Entity::PLACEHOLDER {
        commands.entity(game.background).despawn();
    }
    
    // 清理所有格子和棋子
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            if game.pieces[x][y] != Entity::PLACEHOLDER {
                commands.entity(game.pieces[x][y]).despawn();
            }
            if game.cells[x][y] != Entity::PLACEHOLDER {
                commands.entity(game.cells[x][y]).despawn();
            }
        }
    }
}