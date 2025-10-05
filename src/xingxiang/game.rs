use bevy::prelude::*;

use crate::{
    general::board::UpdateBoard, graphics::{entity::{CellCom, Shape}, interaction::{ClickEvent, DragEvent}}, 
    tree::game_tree::GameTree, 
    xingxiang::{draw::*, general::*, utils::*},
};

enum GameState {
    S1,
    S2(usize, usize),
    S3(usize, usize, usize, usize),
}

#[derive(Component)]
pub struct XingxiangGame {
    pub board: XingxiangBoard,
    pub tree: GameTree<XingxiangBoard>,
    rect: Rect,
    cells: Vec<Vec<Entity>>,
    pieces: Vec<Vec<Entity>>,
    background: Entity,
    updated: bool,
    state: GameState,
    new_piece: Entity,
    dark_overlay: Entity,
    promotion_choices: Vec<Entity>,
}

impl XingxiangGame {
    pub fn new() -> Self {
        Self {
            board: XingxiangBoard::default(),
            tree: GameTree::new(XingxiangBoard::default()),
            rect: Rect::from_center_size(Vec2::ZERO, Vec2::new(500.0, 500.0)),
            cells: vec![vec![Entity::PLACEHOLDER; BOARD_SIZE_J]; BOARD_SIZE_I],
            pieces: vec![vec![Entity::PLACEHOLDER; BOARD_SIZE_J]; BOARD_SIZE_I],
            background: Entity::PLACEHOLDER,
            updated: false,
            state: GameState::S1,
            new_piece: Entity::PLACEHOLDER,
            dark_overlay: Entity::PLACEHOLDER,
            promotion_choices: Vec::new(),
        }
    }
    pub fn try_move(&mut self, step: XingxiangStep) {
        if self.tree.try_move(step) {
            self.updated = false;
            self.board = self.tree.board();
        }
    }
}

fn piece_sprite(p: XingxiangPiece, cell_size: Vec2, textures: &XingxiangTextureAssets) -> Sprite {
    let mut sprite = Sprite::from_image(match p.color {
        XingxiangPieceColor::Black => {
            match p.role {
                XingxiangPieceRole::Pawn => textures.black_piece.clone(),
                XingxiangPieceRole::Rook => textures.black_rook.clone(),
                XingxiangPieceRole::Knight => textures.black_knight.clone(),
                XingxiangPieceRole::Bishop => textures.black_bishop.clone(),
                XingxiangPieceRole::King => textures.black_king.clone(),
            }
        },
        XingxiangPieceColor::White => {
            match p.role {
                XingxiangPieceRole::Pawn => textures.white_piece.clone(),
                XingxiangPieceRole::Rook => textures.white_rook.clone(),
                XingxiangPieceRole::Knight => textures.white_knight.clone(),
                XingxiangPieceRole::Bishop => textures.white_bishop.clone(),
                XingxiangPieceRole::King => textures.white_king.clone(),
            }
        },
    });
    sprite.custom_size = Some(cell_size);
    sprite
}

fn draw(
    commands: &mut Commands,
    textures: &XingxiangTextureAssets,
    game: &mut XingxiangGame,
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

    let leftdown = game.rect.center() - game.rect.size() * (7.0 / 16.0);
    let cell_size = game.rect.size() / 8.0;
    let (dx, dy) = (cell_size.x, cell_size.y);
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            let piece = if let Some(p) = game.board.pieces[x][y] {
                let replaced = match game.state {
                    GameState::S1 => false,
                    GameState::S2(x1, y1) => { x == x1 && y == y1 },
                    GameState::S3(x1, y1, _, _) => { x == x1 && y == y1 },
                };
                if replaced {
                    Entity::PLACEHOLDER
                } else {
                    commands.spawn((
                        {
                            piece_sprite(p, cell_size, textures)
                        },
                        {
                            Transform::from_translation((leftdown + Vec2::new(x as f32 * dx, y as f32 * dy)).extend(2.0))
                        }
                    )).id()
                }
            } else {
                Entity::PLACEHOLDER
            };
            game.pieces[x][y] = piece;
            let cell = commands.spawn((
                CellCom {
                    shape: Shape::Rect { rect: Rect::from_center_size(leftdown + Vec2::new(x as f32 * dx, y as f32 * dy), cell_size) },
                    clickable: true,
                    dragable: false,
                    upper_piece: Entity::PLACEHOLDER,
                },
            )).id();
            game.cells[x][y] = cell;
        }
    }

    if let GameState::S2(x1, y1) = game.state {
        game.new_piece = commands.spawn((
            {
                piece_sprite(XingxiangPiece {
                    color: game.board.active_player,
                    role: XingxiangPieceRole::Pawn,
                }, cell_size, textures)
            },
            {
                Transform::from_translation((leftdown + Vec2::new(x1 as f32 * dx, y1 as f32 * dy)).extend(2.0))
            }
        )).id();
    }

    if let GameState::S3(x1, y1, x2, y2) = game.state {
        game.new_piece = commands.spawn((
            {
                piece_sprite(XingxiangPiece {
                    color: game.board.active_player,
                    role: XingxiangPieceRole::Pawn,
                }, cell_size, textures)
            },
            {
                Transform::from_translation((leftdown + Vec2::new(x1 as f32 * dx, y1 as f32 * dy)).extend(2.0))
            }
        )).id();
        game.dark_overlay = commands.spawn((
            {
                Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.7), game.rect.size())
            },
            {
                Transform::from_translation(game.rect.center().extend(3.0))
            }
        )).id();
        let pro_choices = game.board.promotion_choices((x1, y1), (x2, y2));
        for (i, c) in pro_choices.iter().enumerate() {
            game.promotion_choices.push(
                commands.spawn((
                    {
                        piece_sprite(*c, cell_size, textures)
                    },
                    {
                        let x = x2;
                        let y = if y2 < 4 {
                            y2 + i + 1
                        } else {
                            y2 - i - 1
                        };
                        Transform::from_translation((leftdown + Vec2::new(x as f32 * dx, y as f32 * dy)).extend(4.0))
                    },
                )).id()
            );
        }
    }
}

fn clear(
    commands: &mut Commands,
    game: &mut XingxiangGame,
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
    if game.new_piece != Entity::PLACEHOLDER {
        commands.entity(game.new_piece).despawn();
        game.new_piece = Entity::PLACEHOLDER;
    }
    if game.dark_overlay != Entity::PLACEHOLDER {
        commands.entity(game.dark_overlay).despawn();
        game.dark_overlay = Entity::PLACEHOLDER;
    }
    for e in game.promotion_choices.drain(..) {
        commands.entity(e).despawn();
    }
}

fn update(
    commands: &mut Commands,
    game: &mut XingxiangGame,
    _er_drag: &mut EventReader<DragEvent>,
    er_click: &mut EventReader<ClickEvent>,
    er_update: &mut EventReader<UpdateBoard<XingxiangBoard>>,
    textures: &XingxiangTextureAssets,
) {
    if !game.board.end {
        for event in er_click.read() {
            for x in 0..BOARD_SIZE_I {
                for y in 0..BOARD_SIZE_J {
                    if game.cells[x][y] == event.cell {
                        match game.state {
                            GameState::S1 => {
                                if game.board.valid_pos1((x, y)) {
                                    game.updated = false;
                                    game.state = GameState::S2(x, y);
                                }
                            },
                            GameState::S2(x1, y1) => {
                                if game.board.promotion_choices((x1, y1), (x, y)).is_empty() {
                                    game.try_move(XingxiangStep {
                                        pos: (x1, y1),
                                        change: None,
                                    });
                                    game.state = GameState::S1;
                                } else {
                                    game.updated = false;
                                    game.state = GameState::S3(x1, y1, x, y);
                                }
                            },
                            GameState::S3(x1, y1, x2, y2) => {
                                let pro_choices = game.board.promotion_choices((x1, y1), (x2, y2));
                                let cond = if y2 < 4 {
                                    x == x2 && y > y2 && y - y2 <= pro_choices.len()
                                } else {
                                    x == x2 && y < y2 && y2 - y <= pro_choices.len()
                                };
                                if cond {
                                    let p = if y2 < 4 {
                                        pro_choices[y - y2 - 1]
                                    } else {
                                        pro_choices[y2 - y - 1]
                                    };
                                    game.try_move(XingxiangStep {
                                        pos: (x1, y1),
                                        change: Some(((x2, y2), p)),
                                    });
                                    game.state = GameState::S1;
                                } else {
                                    game.updated = false;
                                    game.state = GameState::S2(x1, y1);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
    
    for event in er_update.read() {
        game.updated = false;
        game.board = event.new_board.clone();
        game.state = GameState::S1;
    }

    if !game.updated {
        clear(commands, game);
        draw(commands, textures, game);
        game.updated = true;
    }
}

pub fn xingxiang_update(
    mut commands: Commands,
    mut q_game: Query<&mut XingxiangGame>,
    mut er_drag: EventReader<DragEvent>,    
    mut er_click: EventReader<ClickEvent>,
    mut er_update: EventReader<UpdateBoard<XingxiangBoard>>,
    textures: Res<XingxiangTextureAssets>,
) {
    for mut game in q_game.iter_mut() {
        update(&mut commands, &mut game, &mut er_drag, &mut er_click, &mut er_update, &textures);
    }
}

#[derive(Event)]
pub struct EndXingxiangGame {
    pub game_entity: Entity,
}

// 处理游戏结束的系统
pub fn handle_end_xingxiang_game(
    mut commands: Commands,
    mut end_events: EventReader<EndXingxiangGame>,
    mut q_game: Query<&mut XingxiangGame>,
) {
    for event in end_events.read() {
        if let Ok(mut game) = q_game.get_mut(event.game_entity) {
            // 先清理所有子实体
            clear(&mut commands, &mut game);
            
            // 然后删除游戏实体本身
            commands.entity(event.game_entity).despawn();
        }
    }
}