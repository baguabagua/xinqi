use crate::general::*;
use crate::hequn::utils::*;

#[derive(Clone, Copy, PartialEq)]
pub enum HequnPiece {
    Black,
    White,
}

impl HequnPiece {
    fn flip(&self) -> Self {
        match self {
            HequnPiece::Black => HequnPiece::White,
            HequnPiece::White => HequnPiece::Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum HequnStep {
    Pos(usize, usize),
    Pass,
}

impl Step for HequnStep {}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum HequnCell {
    #[default]
    Grey,
    Colored(HequnPiece),
}

#[derive(Clone)]
pub struct HequnBoard {
    pub pieces: Vec<Vec<Option<HequnPiece>>>,
    pub cells: Vec<Vec<HequnCell>>,
    pub active_player: HequnPiece,
    pub last_pass: bool,
    pub end: bool,
    pub black_score: usize,
    pub white_score: usize,
    pub fullmove: usize,
}

impl Default for HequnBoard {
    fn default() -> Self {
        Self { 
            pieces: (0..BOARD_SIZE_I).map(|_| (0..BOARD_SIZE_J).map(|_| None).collect()).collect(), 
            cells: (0..BOARD_SIZE_I).map(|_| (0..BOARD_SIZE_J).map(|_| HequnCell::default()).collect()).collect(), 
            active_player: HequnPiece::Black, 
            last_pass: false,
            end: false,
            black_score: 0,
            white_score: 0,
            fullmove: 1,
        }
    }
}

impl Board for HequnBoard {
    type S = HequnStep;

    fn try_move(&self, step: Self::S) -> Option<Self> where Self: Sized {
        if self.end {
            return None
        }
        match step {
            HequnStep::Pos(x, y) => {
                if x > BOARD_SIZE_I || y > BOARD_SIZE_J {
                    return None
                }
                match self.pieces[x][y] {
                    Some(_) => None,
                    None => {
                        let mut pieces = self.pieces.clone();
                        pieces[x][y] = Some(self.active_player);

                        let mut cells = self.cells.clone();
                        for (d1, d2, d3) in PAINT_OFFSET {
                            let Some((x1, y1)) = add_offset((x, y), d1) else {
                                continue;
                            };
                            let Some((x2, y2)) = add_offset((x, y), d2) else {
                                continue;
                            };
                            let Some((x3, y3)) = add_offset((x, y), d3) else {
                                continue;
                            };
                            if self.pieces[x1][y1] == Some(self.active_player) && self.pieces[x2][y2] == Some(self.active_player) {
                                for d in OFFSET {
                                    if let Some((xt, yt)) = add_offset((x3, y3), d) {
                                        cells[xt][yt] = HequnCell::Colored(self.active_player);
                                    }
                                }
                            }
                        }

                        let mut black_score = 0;
                        let mut white_score = 0;
                        for x in 0..BOARD_SIZE_I {
                            for y in 0..BOARD_SIZE_J {
                                match cells[x][y] {
                                    HequnCell::Grey => {},
                                    HequnCell::Colored(HequnPiece::Black) => { black_score += 1; },
                                    HequnCell::Colored(HequnPiece::White) => { white_score += 1; },
                                }
                            }
                        }

                        Some(Self {
                            pieces,
                            cells,
                            active_player: self.active_player.flip(),
                            last_pass: false,
                            end: false,
                            black_score,
                            white_score,
                            fullmove: match self.active_player {
                                HequnPiece::Black => self.fullmove,
                                HequnPiece::White => self.fullmove + 1,
                            },
                        })
                    },
                }
            },
            HequnStep::Pass => {
                Some(Self {
                    active_player: self.active_player.flip(),
                    last_pass: true,
                    end: self.last_pass,
                    fullmove: match self.active_player {
                        HequnPiece::Black => self.fullmove,
                        HequnPiece::White => self.fullmove + 1,
                    },
                    ..self.clone()
                })
            },
        }
    }

    fn all_move(&self) -> Vec<Self::S> {
        if self.end {
            return Vec::new();
        }
        (0..BOARD_SIZE_I).flat_map(|x| {
            (0..BOARD_SIZE_J).filter_map(move |y| {
                match self.pieces[x][y] {
                    Some(_) => { None },
                    None => { Some(Self::S::Pos(x, y)) },
                }
            })
        }).collect()
    }

    fn end_game(&self) -> bool {
        return self.end;
    }

    fn game_info(&self) -> &str {
        if self.end {
            if self.black_score > self.white_score {
                "Black Win"
            } else if self.black_score == self.white_score {
                "Draw"
            } else {
                "White Win"
            }
        } else {
            match self.active_player {
                HequnPiece::Black => "Black Play",
                HequnPiece::White => "White Play",
            }
        }
    }

    fn read_step(&self, s: String) -> Option<Self::S> {
        if self.end {
            return None;
        }
        if s == "pass" {
            return Some(HequnStep::Pass);
        }

        let mut chars = s.chars();

        if let Some(first_char) = chars.next() {
            let first_char = first_char as u8;
            if first_char >= b'a' && first_char < b'a' + BOARD_SIZE_I as u8 {
                let x = (first_char - b'a') as usize;
                let num_str: String = chars.collect();
                if let Ok(y) = num_str.parse::<usize>() {
                    if y == 0 { return None } // 由于类型限制必须单独处理
                    let y = y - 1;
                    if y < BOARD_SIZE_J && self.pieces[x][y].is_none() {
                        return Some(HequnStep::Pos(x, y));
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
            HequnStep::Pos(x, y) => {
                if x >= BOARD_SIZE_I || y >= BOARD_SIZE_J || self.pieces[x][y].is_some() {
                    None 
                } else {
                    Some(format!("{}{}", (b'a' + x as u8) as char, y + 1))
                }
            }
            HequnStep::Pass => Some("pass".to_string()),
        }
    }

    fn read_fen(s: String) -> Option<Self> where Self: Sized {
        if !s.starts_with("[hequn] ") {
            return None;
        }
        
        let parts: Vec<&str> = s.split_whitespace().collect();
        // 应该有5个部分: [hequn], pieces, cells, active_player, last_pass
        if parts.len() != 5 {
            return None;
        }
        
        // 解析pieces部分
        let pieces_str = parts[1];
        let pieces_rows: Vec<&str> = pieces_str.split('/').collect();
        if pieces_rows.len() != BOARD_SIZE_I {
            return None;
        }
        
        let mut pieces: Vec<Vec<Option<HequnPiece>>> = vec![vec![None; BOARD_SIZE_I]; BOARD_SIZE_J];
        for (i, row_str) in pieces_rows.iter().enumerate() {
            if row_str.len() != BOARD_SIZE_J {
                return None;
            }
            for (j, ch) in row_str.chars().enumerate() {
                pieces[i][j] = match ch {
                    'b' => Some(HequnPiece::Black),
                    'w' => Some(HequnPiece::White),
                    '-' => None,
                    _ => return None, // 无效字符
                };
            }
        }
        
        // 解析cells部分
        let cells_str = parts[2];
        let cells_rows: Vec<&str> = cells_str.split('/').collect();
        if cells_rows.len() != BOARD_SIZE_I {
            return None;
        }

        let mut black_score = 0;
        let mut white_score = 0;
        
        let mut cells: Vec<Vec<HequnCell>> = vec![vec![HequnCell::Grey; BOARD_SIZE_I]; BOARD_SIZE_J];
        for (i, row_str) in cells_rows.iter().enumerate() {
            if row_str.len() != BOARD_SIZE_J {
                return None;
            }
            for (j, ch) in row_str.chars().enumerate() {
                cells[i][j] = match ch {
                    'b' => { 
                        black_score += 1;
                        HequnCell::Colored(HequnPiece::Black)
                    },
                    'w' => { 
                        white_score += 1;
                        HequnCell::Colored(HequnPiece::White)
                    },
                    '-' => HequnCell::Grey,
                    _ => return None, // 无效字符
                };
            }
        }
        
        // 解析当前玩家
        let active_player = match parts[3] {
            "b" => HequnPiece::Black,
            "w" => HequnPiece::White,
            _ => return None,
        };
        
        // 解析最后一步是否是pass
        let last_pass = match parts[4] {
            "1" => true,
            "0" => false,
            _ => return None,
        };
        
        // 创建并返回游戏状态实例
        Some(Self {
            pieces,
            cells,
            active_player,
            last_pass,
            end: false,
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
                        Some(HequnPiece::Black) => 'b',
                        Some(HequnPiece::White) => 'w',
                        None => '-',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("/");

        let cells = self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        HequnCell::Colored(HequnPiece::Black) => 'b',
                        HequnCell::Colored(HequnPiece::White) => 'w',
                        HequnCell::Grey => '-',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("/");

        let active_player = match self.active_player {
            HequnPiece::Black => "b",
            HequnPiece::White => "w",
        };

        let last_pass = if self.last_pass { "1" } else { "0" };

        format!("[hequn] {} {} {} {}", pieces, cells, active_player, last_pass)
    }
    
    fn get_fullmove(&self) -> usize {
        self.fullmove
    }
    
    fn get_active_player(&self) -> PlayerOrder {
        match self.active_player {
            HequnPiece::Black => PlayerOrder::First,
            HequnPiece::White => PlayerOrder::Second,
        }
    }
}