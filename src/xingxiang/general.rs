use crate::general::*;
use crate::xingxiang::utils::*;

#[derive(Clone, Copy, PartialEq)]
pub struct XingxiangStep {
    pub pos: (usize, usize),
    pub change: Option<((usize, usize), XingxiangPiece)>,
}

impl Step for XingxiangStep {}

#[derive(Clone)]
pub struct XingxiangBoard {
    pub pieces: Vec<Vec<Option<XingxiangPiece>>>,
    pub active_player: XingxiangPieceColor,
    pub end: bool,
    pub winner: Option<XingxiangPieceColor>,
    pub fullmove: usize,
}

impl XingxiangBoard {
    pub fn valid_pos1(&self, (x, y): (usize, usize)) -> bool {
        valid_coordinate(x, y) && self.pieces[x][y].is_none_or(|p| p.color == self.active_player)
    }
    pub fn promotion_choices(&self, pos: (usize, usize), pos_pro: (usize, usize)) -> Vec<XingxiangPiece> {
        let mut res = Vec::new();

        for role in ADVANCED_PIECES {
            let piece = XingxiangPiece {
                color: self.active_player,
                role,
            };
            if self.try_move(XingxiangStep {
                pos,
                change: Some((pos_pro, piece)),
            }).is_some() {
                res.push(piece);
            }
        }

        res
    }
}

impl Default for XingxiangBoard {
    fn default() -> Self {
        Self { 
            pieces: vec![vec![None; BOARD_SIZE_I]; BOARD_SIZE_J], 
            active_player: XingxiangPieceColor::Black, 
            end: false, 
            winner: None,
            fullmove: 1, 
        }
    }
}

fn can_promote(
    pieces: &Vec<Vec<Option<XingxiangPiece>>>, 
    (x, y): (usize, usize), 
    role: XingxiangPieceRole, 
    player: XingxiangPieceColor,
) -> bool {
    if pieces[x][y].is_none_or(|p| p.color != player) {
        return false;
    }
    let mut score = 0;
    let offsets = role.offsets();
    for offset in offsets {
        if let Some((xpp, ypp)) = add_offset((x, y), offset) {
            if pieces[xpp][ypp].is_some_and(|p| p.color == player) {
                score += 1;
            }
        }
    }
    score >= 3
}

fn find_king_pos(pieces: &Vec<Vec<Option<XingxiangPiece>>>, player: XingxiangPieceColor) -> Option<(usize, usize)> {
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            if pieces[x][y].is_some_and(|p| {
                p.color == player && p.role == XingxiangPieceRole::King
            }) {
                return Some((x, y));
            }
        }
    }
    None
}

fn try_eat(pieces: &mut Vec<Vec<Option<XingxiangPiece>>>, eat_pos: (usize, usize), player: XingxiangPieceColor) {
    let mut seen = Vec::new();
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            if let Some(p) = pieces[x][y] {
                if p.color == player.flip() {
                    continue;
                }
                let offset = diff((x, y), eat_pos);
                let offsets = p.role.offsets();
                if offsets.contains(&offset) && !seen.contains(&p.role) {
                    seen.push(p.role);
                }
            }
        }
    }
    if seen.len() >= 3 {
        pieces[eat_pos.0][eat_pos.1] = None
    }
}

fn can_eat_king(pieces: &Vec<Vec<Option<XingxiangPiece>>>, king_pos: (usize, usize), player: XingxiangPieceColor) -> bool {
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            if let Some(p) = pieces[x][y] {
                if p.color == player.flip() {
                    continue;
                }
                let offset = diff((x, y), king_pos);
                let offsets = p.role.offsets();
                if offsets.contains(&offset) {
                    return true
                }
            }
        }
    }
    false
}

impl Board for XingxiangBoard {
    type S = XingxiangStep;

    fn try_move(&self, step: Self::S) -> Option<Self> where Self: Sized {
        if self.end {
            return None
        }
        let mut pieces = self.pieces.clone();
        let (x, y) = step.pos;
        if !valid_coordinate(x, y) {
            return None 
        }
        if pieces[x][y].is_some_and(|p| p.color == self.active_player.flip()) {
            return None 
        }
        pieces[x][y] = Some(XingxiangPiece {
            role: XingxiangPieceRole::Pawn,
            color: self.active_player,
        });
        if let Some(((xp, yp), target)) = step.change {
            if !valid_coordinate(xp, yp) || target.color == self.active_player.flip() {
                return None 
            }
            if pieces[xp][yp].is_none_or(|p| p.color == self.active_player.flip()) {
                return None 
            }
            
            let offsets = if target.role == XingxiangPieceRole::Pawn {
                return None;
            } else {
                target.role.offsets()
            };
            let d = diff((x, y), (xp, yp));
            if !offsets.contains(&d) && d != (0, 0) {
                return None 
            }
            if can_promote(&pieces, (xp, yp), target.role, self.active_player) {
                // 如果生成王，则先将之前的王降为普通棋子
                if target.role == XingxiangPieceRole::King {
                    for xpp in 0..BOARD_SIZE_I {
                        for ypp in 0..BOARD_SIZE_J {
                            if let Some(p) = &mut pieces[xpp][ypp] {
                                if p.color == self.active_player && p.role == XingxiangPieceRole::King {
                                    p.role = XingxiangPieceRole::Pawn;
                                }
                            }
                        }
                    }
                }
                pieces[xp][yp] = Some(target);
                // 吃子
                for offset in &offsets {
                    if let Some((xpp, ypp)) = add_offset((xp, yp), *offset) {
                        if pieces[xpp][ypp].is_some_and(|p| p.color == self.active_player.flip()) {
                            try_eat(&mut pieces, (xpp, ypp), self.active_player);
                        }
                    }
                }
            } else {
                return None
            }
        }
        // 对家尝试吃王
        if let Some(kp) = find_king_pos(&pieces, self.active_player) {
            if can_eat_king(&pieces, kp, self.active_player.flip()) {
                return Some(Self {
                    pieces,
                    active_player: self.active_player.flip(),
                    end: true,
                    winner: Some(self.active_player.flip()),
                    fullmove: match self.active_player {
                        XingxiangPieceColor::Black => self.fullmove,
                        XingxiangPieceColor::White => self.fullmove + 1,
                    },
                });
            }
        } else if self.fullmove >= 8 { // 第八回合后没有王直接判负，感谢 AI 发现的 bug (之前是只在等于时判断)
            return Some(Self { 
                pieces, 
                active_player: self.active_player.flip(), 
                end: true, 
                winner: Some(self.active_player.flip()),
                fullmove: match self.active_player {
                    XingxiangPieceColor::Black => self.fullmove,
                    XingxiangPieceColor::White => self.fullmove + 1,
                }, 
            });
        }
        return Some(Self { 
            pieces, 
            active_player: self.active_player.flip(), 
            end: false, 
            winner: None, 
            fullmove: match self.active_player {
                XingxiangPieceColor::Black => self.fullmove,
                XingxiangPieceColor::White => self.fullmove + 1,
            }, 
        });
    }

    fn all_move(&self) -> Vec<Self::S> {
        if self.end {
            return Vec::new();
        }
        let mut res = Vec::new();
        let mut pieces = self.pieces.clone();
        for x in 0..BOARD_SIZE_I {
            for y in 0..BOARD_SIZE_J {
                if self.pieces[x][y].is_some_and(|p| p.color == self.active_player.flip()) {
                    continue;
                }
                let p = pieces[x][y].take();
                pieces[x][y] = Some(XingxiangPiece {
                    role: XingxiangPieceRole::Pawn,
                    color: self.active_player,
                });
                res.push(XingxiangStep { pos: (x, y), change: None });
                for role in ADVANCED_PIECES {
                    if can_promote(&pieces, (x, y), role, self.active_player) {
                        res.push(XingxiangStep { pos: (x, y), change: Some(((x, y), XingxiangPiece { role, color: self.active_player })) });
                    }
                    let offsets = role.offsets();
                    for offset in offsets {
                        if let Some((xp, yp)) = add_offset((x, y), offset) {
                            if can_promote(&pieces, (xp, yp), role, self.active_player) {
                                res.push(XingxiangStep { pos: (x, y), change: Some(((xp, yp), XingxiangPiece { role, color: self.active_player })) });
                            }
                        }
                    }
                }
                pieces[x][y] = p;
            }
        }
        res
    }

    fn end_game(&self) -> bool {
        self.end
    }

    fn get_winner(&self) -> Option<PlayerOrder> {
        match self.winner {
            Some(winner) => match winner {
                XingxiangPieceColor::Black => Some(PlayerOrder::First),
                XingxiangPieceColor::White => Some(PlayerOrder::Second),
            },
            None => None,
        }
    }

    fn game_info(&self) -> &str {
        match self.end {
            true => {
                match self.winner {
                    Some(winner) => match winner {
                        XingxiangPieceColor::Black => "Black Win",
                        XingxiangPieceColor::White => "White Win",
                    },
                    None => "Draw",
                }
            },
            false => {
                match self.active_player {
                    XingxiangPieceColor::Black => "Black Play",
                    XingxiangPieceColor::White => "White Play",
                }
            },
        }
    }

    fn get_fullmove(&self) -> usize {
        self.fullmove
    }

    fn get_active_player(&self) -> PlayerOrder {
        match self.active_player {
            XingxiangPieceColor::Black => PlayerOrder::First,
            XingxiangPieceColor::White => PlayerOrder::Second,
        }
    }

    fn write_step(&self, step: Self::S) -> Option<String> {
        if self.try_move(step).is_none() {
            return None 
        }
        let (x, y) = step.pos;
        let change = match step.change {
            Some(((cx, cy), p)) => {
                format!("{}{}{}", p, (b'a' + cx as u8) as char, cy + 1)
            },
            None => String::new(),
        };

        Some(format!("{}{}{}", (b'a' + x as u8) as char, y + 1, change))
    }

    fn read_step(&self, s: String) -> Option<Self::S> {
        if self.end {
            return None;
        }

        let mut chars = s.chars();
        
        // 解析位置部分 (如 "a1")
        if let Some(first_char) = chars.next() {
            let first_char = first_char as u8;
            if first_char >= b'a' && first_char < b'a' + BOARD_SIZE_I as u8 {
                let x = (first_char - b'a') as usize;
                let num_str: String = chars.by_ref().take_while(|c| c.is_digit(10)).collect();
                if let Ok(y) = num_str.parse::<usize>() {
                    if y == 0 { return None }
                    let y = y - 1;
                    if y < BOARD_SIZE_J {
                        // 检查是否有变化部分
                        let remaining: String = chars.collect();
                        if remaining.is_empty() {
                            // 只有位置，没有变化
                            return Some(XingxiangStep {
                                pos: (x, y),
                                change: None,
                            });
                        } else {
                            // 解析变化部分 (如 "Ra2")
                            if remaining.len() >= 3 {
                                let role_char = remaining.chars().next().unwrap();
                                let role = match role_char {
                                    'R' | 'r' => XingxiangPieceRole::Rook,
                                    'N' | 'n' => XingxiangPieceRole::Knight,
                                    'B' | 'b' => XingxiangPieceRole::Bishop,
                                    'K' | 'k' => XingxiangPieceRole::King,
                                    _ => return None,
                                };
                                
                                let mut change_chars = remaining[1..].chars();
                                if let Some(change_first_char) = change_chars.next() {
                                    let change_first_char = change_first_char as u8;
                                    if change_first_char >= b'a' && change_first_char < b'a' + BOARD_SIZE_I as u8 {
                                        let xp = (change_first_char - b'a') as usize;
                                        let change_num_str: String = change_chars.collect();
                                        if let Ok(yp) = change_num_str.parse::<usize>() {
                                            if yp == 0 { return None }
                                            let yp = yp - 1;
                                            if yp < BOARD_SIZE_J {
                                                let color = if role_char.is_uppercase() {
                                                    XingxiangPieceColor::White
                                                } else {
                                                    XingxiangPieceColor::Black
                                                };
                                                
                                                return Some(XingxiangStep {
                                                    pos: (x, y),
                                                    change: Some(((xp, yp), XingxiangPiece {
                                                        role,
                                                        color,
                                                    })),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn read_fen(s: String) -> Option<Self> where Self: Sized {
        if !s.starts_with("[xingxiang] ") {
            return None;
        }
        
        let parts: Vec<&str> = s.split_whitespace().collect();
        // 应该有4个部分: [xingxiang], pieces, active_player, fullmove
        if parts.len() != 4 {
            return None;
        }
        
        // 解析pieces部分
        let pieces_str = parts[1];
        let pieces_rows: Vec<&str> = pieces_str.split('/').collect();
        if pieces_rows.len() != BOARD_SIZE_I {
            return None;
        }
        
        let mut pieces: Vec<Vec<Option<XingxiangPiece>>> = vec![vec![None; BOARD_SIZE_I]; BOARD_SIZE_J];
        for (i, row_str) in pieces_rows.iter().enumerate() {
            if row_str.len() != BOARD_SIZE_J {
                return None;
            }
            for (j, ch) in row_str.chars().enumerate() {
                pieces[i][j] = match ch {
                    'P' => Some(XingxiangPiece { role: XingxiangPieceRole::Pawn, color: XingxiangPieceColor::White }),
                    'R' => Some(XingxiangPiece { role: XingxiangPieceRole::Rook, color: XingxiangPieceColor::White }),
                    'N' => Some(XingxiangPiece { role: XingxiangPieceRole::Knight, color: XingxiangPieceColor::White }),
                    'B' => Some(XingxiangPiece { role: XingxiangPieceRole::Bishop, color: XingxiangPieceColor::White }),
                    'K' => Some(XingxiangPiece { role: XingxiangPieceRole::King, color: XingxiangPieceColor::White }),
                    'p' => Some(XingxiangPiece { role: XingxiangPieceRole::Pawn, color: XingxiangPieceColor::Black }),
                    'r' => Some(XingxiangPiece { role: XingxiangPieceRole::Rook, color: XingxiangPieceColor::Black }),
                    'n' => Some(XingxiangPiece { role: XingxiangPieceRole::Knight, color: XingxiangPieceColor::Black }),
                    'b' => Some(XingxiangPiece { role: XingxiangPieceRole::Bishop, color: XingxiangPieceColor::Black }),
                    'k' => Some(XingxiangPiece { role: XingxiangPieceRole::King, color: XingxiangPieceColor::Black }),
                    '-' => None,
                    _ => return None, // 无效字符
                };
            }
        }
        
        // 解析当前玩家
        let active_player = match parts[2] {
            "b" => XingxiangPieceColor::Black,
            "w" => XingxiangPieceColor::White,
            _ => return None,
        };
        
        // 解析回合数
        let fullmove = match parts[3].parse::<usize>() {
            Ok(n) => n,
            Err(_) => return None,
        };
        
        // 创建并返回游戏状态实例
        Some(Self {
            pieces,
            active_player,
            end: false,
            winner: None,
            fullmove,
        })
    }

    fn write_fen(&self) -> String {
        let pieces = self.pieces
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Some(p) => p.to_string(),
                        None => String::from("-"),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("/");

        let active_player = match self.active_player {
            XingxiangPieceColor::Black => "b",
            XingxiangPieceColor::White => "w",
        };

        format!("[xingxiang] {} {} {}", pieces, active_player, self.fullmove)
    }
}