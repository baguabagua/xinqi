use crate::general::*;
use crate::zhandi::utils::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ZhandiPiece {
    Black,
    White,
}

impl ZhandiPiece {
    fn flip(&self) -> Self {
        match self {
            ZhandiPiece::Black => ZhandiPiece::White,
            ZhandiPiece::White => ZhandiPiece::Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ZhandiStep {
    Pos(usize, usize),
}

impl Step for ZhandiStep {}

#[derive(Clone)]
pub struct ZhandiBoard {
    pub pieces: Vec<Vec<Option<ZhandiPiece>>>,
    pub active_player: ZhandiPiece,
    pub end: bool,
    pub black_score: usize,
    pub white_score: usize,
    pub fullmove: usize,
}

impl Default for ZhandiBoard {
    fn default() -> Self {
        Self { 
            pieces: (0..BOARD_DIAMETER).map(|_| (0..BOARD_DIAMETER).map(|_| None).collect()).collect(), 
            active_player: ZhandiPiece::Black, 
            end: false,
            black_score: 0,
            white_score: 0,
            fullmove: 1,
        }
    }
}

fn try_control(pieces: &mut Vec<Vec<Option<ZhandiPiece>>>, player: ZhandiPiece) {
    let mut changed = true;

    while changed {
        changed = false;

        for x in 0..BOARD_DIAMETER {
            for y in 0..BOARD_DIAMETER {
                if pieces[x][y] == Some(player.flip()) {
                    let mut player_count = 0;
                    let mut opponent_count = 0;
                    for d in OFFSET {
                        if let Some((xt, yt)) = add_offset((x, y), d) {
                            match pieces[xt][yt] {
                                Some(p) => {
                                    if p == player {
                                        player_count += 1;
                                    } else {
                                        opponent_count += 1;
                                    }
                                },
                                None => {},
                            }
                        }
                    }
                    if player_count > opponent_count {
                        pieces[x][y] = Some(player);
                        changed = true;
                    }
                }
            }
        }
    }
}

impl Board for ZhandiBoard {
    type S = ZhandiStep;

    fn try_move(&self, step: Self::S) -> Option<Self> where Self: Sized {
        if self.end {
            return None 
        }
        match step {
            ZhandiStep::Pos(x, y) => {
                if !valid_coordinate(x, y) || self.pieces[x][y].is_some() {
                    return None 
                }
                let mut pieces = self.pieces.clone();
                pieces[x][y] = Some(self.active_player);

                try_control(&mut pieces, self.active_player.flip());
                
                let end = !((0..BOARD_DIAMETER).any(|x| {
                    (0..BOARD_DIAMETER).any(|y| {
                        valid_coordinate(x, y) && pieces[x][y].is_none()
                    })
                }));
                let black_score = pieces.iter().flatten().flatten().filter(|&&p| p == ZhandiPiece::Black).count();
                let white_score = pieces.iter().flatten().flatten().filter(|&&p| p == ZhandiPiece::White).count();

                Some(ZhandiBoard { 
                    pieces, 
                    active_player: self.active_player.flip(), 
                    end,
                    black_score, 
                    white_score, 
                    fullmove: match self.active_player {
                        ZhandiPiece::Black => self.fullmove,
                        ZhandiPiece::White => self.fullmove + 1,
                    },
                })
            },
        }
    }

    fn all_move(&self) -> Vec<Self::S> {
        if self.end {
            return Vec::new();
        }
        (0..BOARD_DIAMETER).flat_map(|x| {
            (0..BOARD_DIAMETER).filter_map(move |y| {
                if valid_coordinate(x, y) && self.pieces[x][y] == None {
                    Some(Self::S::Pos(x, y))
                } else {
                    None
                }
            })
        }).collect()
    }

    fn end_game(&self) -> bool {
        self.end
    }

    fn get_winner(&self) -> Option<PlayerOrder> {
        if self.black_score > self.white_score + 4 {
            Some(PlayerOrder::First)
        } else if self.black_score == self.white_score + 4 {
            None
        } else {
            Some(PlayerOrder::Second)
        }
    }

    fn game_info(&self) -> &str {
        if self.end {
            if self.black_score > self.white_score + 4 {
                "Black Win"
            } else if self.black_score == self.white_score + 4 {
                "Draw"
            } else {
                "White Win"
            }
        } else {
            match self.active_player {
                ZhandiPiece::Black => "Black Play",
                ZhandiPiece::White => "White Play",
            }
        }
    }

    fn get_fullmove(&self) -> usize {
        self.fullmove
    }

    fn get_active_player(&self) -> PlayerOrder {
        match self.active_player {
            ZhandiPiece::Black => PlayerOrder::First,
            ZhandiPiece::White => PlayerOrder::Second,
        }
    }

    fn read_step(&self, s: String) -> Option<Self::S> {
        if self.end {
            return None;
        }

        let mut chars = s.chars();

        if let Some(first_char) = chars.next() {
            let first_char = first_char as u8;
            if first_char >= b'a' && first_char < b'a' + BOARD_DIAMETER as u8 {
                let x = (first_char - b'a') as usize;
                let x = BOARD_DIAMETER - 1 - x;
                let num_str: String = chars.collect();
                if let Ok(y) = num_str.parse::<usize>() {
                    if y == 0 { return None } // 由于类型限制必须单独处理
                    let y = y - 1;
                    if x < BOARD_RADIUS {
                        if y < x + BOARD_RADIUS {
                            return Some(ZhandiStep::Pos(x, y));
                        } else {
                            return None;
                        }
                    } else {
                        if x + y < BOARD_DIAMETER + BOARD_RADIUS - 1 {
                            return Some(ZhandiStep::Pos(x, y + (x + 1 - BOARD_RADIUS)));
                        } else {
                            return None;
                        }
                    }
                }
            }
        }

        None
    }

    fn write_step(&self, step: Self::S) -> Option<String> {
        if self.end {
            return None;
        }
        match step {
            ZhandiStep::Pos(x, y) => {
                if !valid_coordinate(x, y) || self.pieces[x][y].is_some() {
                    return None; 
                } else {
                    let y = if x < BOARD_RADIUS { y } else { y - (x + 1 - BOARD_RADIUS) };
                    return Some(format!("{}{}", (b'a' + (BOARD_DIAMETER - 1 - x) as u8) as char, y + 1));
                }
            },
        }
    }

    fn read_fen(s: String) -> Option<Self> where Self: Sized {
        if !s.starts_with("[zhandi] ") {
            return None;
        }
        
        let parts: Vec<&str> = s.split_whitespace().collect();
        // 应该有3个部分: [zhandi], pieces, active_player
        if parts.len() != 3 {
            return None;
        }

        // 解析pieces部分
        let pieces_str = parts[1];
        let pieces_rows: Vec<&str> = pieces_str.split('/').collect();
        if pieces_rows.len() != BOARD_DIAMETER {
            return None;
        }
        
        let mut pieces: Vec<Vec<Option<ZhandiPiece>>> = vec![vec![None; BOARD_DIAMETER]; BOARD_DIAMETER];
        for (i, row_str) in pieces_rows.iter().enumerate() {
            if row_str.len() != BOARD_DIAMETER {
                return None;
            }
            for (j, ch) in row_str.chars().enumerate() {
                if !valid_coordinate(i, j) {
                    pieces[i][j] = None;
                } else {
                    pieces[i][j] = match ch {
                        'b' => Some(ZhandiPiece::Black),
                        'w' => Some(ZhandiPiece::White),
                        '-' => None,
                        _ => return None, // 无效字符
                    }
                }
            }
        }

        // 解析当前玩家
        let active_player = match parts[2] {
            "b" => ZhandiPiece::Black,
            "w" => ZhandiPiece::White,
            _ => return None,
        };

        let end = !((0..BOARD_DIAMETER).any(|x| {
            (0..BOARD_DIAMETER).any(|y| {
                valid_coordinate(x, y) && pieces[x][y].is_none()
            })
        }));
        let black_score = pieces.iter().flatten().flatten().filter(|&&p| p == ZhandiPiece::Black).count();
        let white_score = pieces.iter().flatten().flatten().filter(|&&p| p == ZhandiPiece::White).count();

        // 创建并返回游戏状态实例
        Some(Self {
            pieces,
            active_player,
            end,
            black_score,
            white_score,
            fullmove: 1,
        })
    }

    fn write_fen(&self) -> String {
        let pieces = self.pieces
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Some(ZhandiPiece::Black) => 'b',
                        Some(ZhandiPiece::White) => 'w',
                        None => '-',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("/");

        let active_player = match self.active_player {
            ZhandiPiece::Black => "b",
            ZhandiPiece::White => "w",
        };

        format!("[zhandi] {} {}", pieces, active_player)
    }
}