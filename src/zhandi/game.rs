use bevy::prelude::*;

use crate::{
    general::board::UpdateBoard, graphics::{entity::{CellCom, Shape}, interaction::{ClickEvent, DragEvent}}, 
    tree::game_tree::GameTree, 
    zhandi::{draw::ZhandiTextureAssets, general::*, utils::*},
};

#[derive(Component)]
pub struct ZhandiGame {
    pub board: ZhandiBoard,
    pub tree: GameTree<ZhandiBoard>,
    rect: Rect,
    cells: Vec<Vec<Entity>>,
    pieces: Vec<Vec<Entity>>,
    background: Entity,
    updated: bool,
}

impl ZhandiGame {
    pub fn new() -> Self {
        Self {
            board: ZhandiBoard::default(),
            tree: GameTree::new(ZhandiBoard::default()),
            rect: Rect::from_center_size(Vec2::ZERO, Vec2::new(600.0, 533.0)),
            cells: vec![vec![Entity::PLACEHOLDER; BOARD_DIAMETER]; BOARD_DIAMETER],
            pieces: vec![vec![Entity::PLACEHOLDER; BOARD_DIAMETER]; BOARD_DIAMETER],
            background: Entity::PLACEHOLDER,
            updated: false,
        }
    }
    pub fn try_move(&mut self, step: ZhandiStep) {
        if self.tree.try_move(step) {
            self.updated = false;
            self.board = self.tree.board();
        }
    }
}

// impl Game for ZhandiGame {
//     type B = ZhandiBoard;

//     fn get_tree(&mut self) -> &mut GameTree<Self::B> {
//         &mut self.tree
//     }
// }

fn draw(
    commands: &mut Commands,
    textures: &ZhandiTextureAssets,
    game: &mut ZhandiGame,
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

    let leftup = Vec2::new(game.rect.center().x - game.rect.size().x * (1.25 / 6.0), game.rect.center().y - game.rect.size().x * (2.165 / 6.0));
    let cell_diameter = game.rect.size().x * (0.625 / 6.0);
    let dcell_diameter = game.rect.size().x / 10.0;
    let dcell_size = Vec2::new(cell_diameter, cell_diameter);
    let (xdx, xdy, ydx, ydy) = (-cell_diameter / 2.0, cell_diameter * (3.0_f32.sqrt() / 2.0), cell_diameter, 0.0);

    for x in 0..BOARD_DIAMETER {
        for y in 0..BOARD_DIAMETER {
            if !valid_coordinate(x, y) {
                continue;
            }
            let center = leftup + Vec2::new(x as f32 * xdx + y as f32 * ydx, x as f32 * xdy + y as f32 * ydy);
            let piece = if let Some(p) = game.board.pieces[x][y] {
                commands.spawn((
                    {
                        let mut sprite = Sprite::from_image(match p {
                            ZhandiPiece::Black => textures.black_piece.clone(),
                            ZhandiPiece::White => textures.white_piece.clone(),
                        });
                        sprite.custom_size = Some(dcell_size);
                        sprite
                    },
                    {
                        Transform::from_translation(center.extend(2.0))
                    }
                )).id()
            } else {
                Entity::PLACEHOLDER
            };
            game.pieces[x][y] = piece;
            let cell = commands.spawn((
                CellCom {
                    shape: Shape::Circle { center, radius: dcell_diameter / 2.0 },
                    clickable: true,
                    dragable: false,
                    upper_piece: Entity::PLACEHOLDER,
                },
            )).id();
            game.cells[x][y] = cell;
        }
    }
}

fn clear(
    commands: &mut Commands,
    game: &mut ZhandiGame,
) {
    if game.background != Entity::PLACEHOLDER {
        commands.entity(game.background).despawn();
    }
    for x in 0..BOARD_DIAMETER {
        for y in 0..BOARD_DIAMETER {
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
    game: &mut ZhandiGame,
    _er_drag: &mut EventReader<DragEvent>,
    er_click: &mut EventReader<ClickEvent>,
    er_update: &mut EventReader<UpdateBoard<ZhandiBoard>>,
    textures: &ZhandiTextureAssets,
) {
    for event in er_click.read() {
        for x in 0..BOARD_DIAMETER {
            for y in 0..BOARD_DIAMETER {
                if game.cells[x][y] == event.cell {
                    game.try_move(ZhandiStep::Pos(x, y));
                }
            }
        }
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

pub fn zhandi_update(
    mut commands: Commands,
    mut q_game: Query<&mut ZhandiGame>,
    mut er_drag: EventReader<DragEvent>,    
    mut er_click: EventReader<ClickEvent>,
    mut er_update: EventReader<UpdateBoard<ZhandiBoard>>,
    textures: Res<ZhandiTextureAssets>,
) {
    for mut game in q_game.iter_mut() {
        update(&mut commands, &mut game, &mut er_drag, &mut er_click, &mut er_update, &textures);
    }
}

#[derive(Event)]
pub struct EndZhandiGame {
    pub game_entity: Entity,
}

// 处理游戏结束的系统
pub fn handle_end_zhandi_game(
    mut commands: Commands,
    mut end_events: EventReader<EndZhandiGame>,
    q_game: Query<&mut ZhandiGame>,
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
fn cleanup_game_entities(commands: &mut Commands, game: &ZhandiGame) {
    // 清理背景
    if game.background != Entity::PLACEHOLDER {
        commands.entity(game.background).despawn();
    }
    
    // 清理所有格子和棋子
    for x in 0..BOARD_DIAMETER {
        for y in 0..BOARD_DIAMETER {
            if game.pieces[x][y] != Entity::PLACEHOLDER {
                commands.entity(game.pieces[x][y]).despawn();
            }
            if game.cells[x][y] != Entity::PLACEHOLDER {
                commands.entity(game.cells[x][y]).despawn();
            }
        }
    }
}