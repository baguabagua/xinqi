use bevy::prelude::*;
use bevy_egui::egui::{self, text::Fonts, Align2, FontId, RichText, Sense};
use crate::{general::*, tree::{pgn::parse_pgn, game_tree_event::{DeleteVariationEvent, MoveToNodeEvent}}};

#[derive(Clone)]
struct MoveData {
    ply: usize,     // 步数编号(1,2,3...)
    san: String,    // 标准记法
    player: PlayerOrder,  // 行动方的颜色
}

#[derive(Clone)]
struct GameTreeNode<B> where B: Board {
    board: B,
    sons: Vec<(StepType<B>, usize, MoveData)>, // 默认第一个是主分支
    parent: Option<usize>,
}

impl<B: Board> GameTreeNode<B> {
    fn new(board: B) -> Self {
        GameTreeNode {
            board: board,
            sons: Vec::new(),
            parent: None,
        }
    }
}

#[derive(Component, Default)]
pub struct GameTree<B> where B: Board {
    nodes: Vec<GameTreeNode<B>>,
    root: usize,
    focus: usize,
}

impl<B: Board> GameTree<B> {
    // 下面的特性在当前的稳定版本（rustc 1.85.0）中还不能使用，但可以在 nightly 版本中使用
    // type S = B::S;
    // type S = StepType<B>;

    pub fn new(board: B) -> Self {
        GameTree {
            nodes: vec![GameTreeNode::new(board)],
            root: 0,
            focus: 0,
        }
    }

    pub fn from_pgn(pgn: String) -> Option<Self> {
        let mut board = B::default();
        let mut tree = Self::new(board.clone());

        let steps = parse_pgn(&pgn);

        for step in steps.into_iter() {
            if let Some(s) = board.read_step(step) {
                if tree.try_move(s) {
                    board = tree.board();
                }
            }
        }

        Some(tree)
    }

    pub fn pgn(&self, mut current: usize) -> String {
        let mut path: Vec<usize> = Vec::new();
        let mut sans: Vec<String> = Vec::new();

        while current != self.root {
            if let Some(parent) = self.nodes[current].parent {
                path.push(current);
                current = parent;
            } else {
                unreachable!()
            }
        }
        path.reverse();

        for node in path {
            for (_step, son, move_data) in self.nodes[current].sons.iter() {
                if node == *son {
                    match move_data.player {
                        PlayerOrder::First => sans.push(format!("{}.{}", move_data.ply, move_data.san)),
                        PlayerOrder::Second => sans.push(move_data.san.clone()),
                    }
                    current = *son;
                    break
                }
            }
        }
        sans.join(" ")
    }

    pub fn from_string(s: String) -> Option<Self> {
        let lines: Vec<&str> = s.trim().lines().collect();
    
        if lines.len() < 4 || lines[0] != "[chess game tree]" {
            return None
        }

        let initial_fen = lines[1];
        let nodes_count: usize = lines[2].parse().unwrap_or(0);
        let info_lines = &lines[3..];
        
        let initial_board = B::read_fen(initial_fen.to_string())?;
        let mut tree = GameTree {
            nodes: vec![GameTreeNode::new(B::default()); nodes_count],
            root: 0,
            focus: 0,
        };
        tree.nodes[0] = GameTreeNode::new(initial_board);

        for (node_id, line) in info_lines.iter().enumerate() {
            if node_id >= nodes_count {
                break 
            }

            let move_infos: Vec<&str> = line.split('|').collect();

            for move_info in move_infos {
                // 解析 (son_id, san)
                if let Some(inner) = move_info.strip_prefix('(')
                    .and_then(|s| s.strip_suffix(')')) 
                {
                    let parts: Vec<&str> = inner.splitn(2, ", ").collect();
                    if parts.len() == 2 {
                        let Ok(son_id) = parts[0].parse::<usize>() else {
                            return None
                        };
                        let san = parts[1].to_string();
                        let step = tree.nodes[node_id].board.read_step(san)?;
                        let board = tree.nodes[node_id].board.try_move(step)?;
                        tree.nodes[son_id].board = board;
                        tree.nodes[son_id].parent = Some(node_id);
                        let move_data = MoveData {
                            ply: tree.nodes[node_id].board.get_fullmove(),
                            san: tree.nodes[node_id].board.write_step(step).unwrap(),
                            player: tree.nodes[node_id].board.get_active_player(),
                        };
                        tree.nodes[node_id].sons.push((step, son_id, move_data));
                    }
                }
            }
        }

        Some(tree) 
    }

    pub fn to_string(&self) -> String {
        let title = "[chess game tree]";
        let initial = self.nodes[self.root].board.write_fen();
        let nodes = self.nodes.len();

        let info = self.nodes.iter().map(|node| {
            node.sons.iter().map(|(_step, son, move_data)| {
                format!("({}, {})", son, move_data.san)
            })
            .collect::<Vec<String>>()
            .join("|")
        })
        .collect::<Vec<String>>()
        .join("\n");

        format!("{}\n{}\n{}\n{}", title, initial, nodes, info)
    }

    pub fn board(&self) -> B {
        self.nodes[self.focus].board.clone()
    }

    pub fn is_first_board(&self) -> bool {
        return self.focus == self.root
    }

    pub fn is_last_board(&self) -> bool {
        return self.nodes[self.focus].sons.len() == 0
    }

    pub fn move_to_start(&self, ew_mtn: &mut EventWriter<MoveToNodeEvent>) {
        let mut target = self.focus;
        while let Some(parent) = self.nodes[target].parent {
            target = parent;
        }
        ew_mtn.write(MoveToNodeEvent { node_id: target });
    }

    pub fn move_backward(&self, ew_mtn: &mut EventWriter<MoveToNodeEvent>) {
        let mut target = self.focus;
        if let Some(parent) = self.nodes[target].parent {
            target = parent;
        }
        ew_mtn.write(MoveToNodeEvent { node_id: target });
    }

    pub fn move_forward(&self, ew_mtn: &mut EventWriter<MoveToNodeEvent>) {
        let mut target = self.focus;
        if !self.nodes[target].sons.is_empty() {
            target = self.nodes[target].sons[0].1;
        }
        ew_mtn.write(MoveToNodeEvent { node_id: target });
    }

    pub fn move_to_end(&self, ew_mtn: &mut EventWriter<MoveToNodeEvent>) {
        let mut target = self.focus;
        while !self.nodes[target].sons.is_empty() {
            target = self.nodes[target].sons[0].1;
        }
        ew_mtn.write(MoveToNodeEvent { node_id: target });
    }

    // 由于rust的禁止双重借用的规则被迫用了比较奇怪的写法，实际上函数式会好一些
    pub fn try_move(&mut self, step: B::S) -> bool {
        {
            let node = &self.nodes[self.focus];
            for (s, son, _) in &node.sons {
                if step == *s {
                    self.focus = *son;
                    return true;
                }
            }
        }
        if let Some(board) = self.nodes[self.focus].board.try_move(step) {
            let new_index = self.nodes.len();
            self.nodes.push(GameTreeNode::new(board));
            self.nodes[new_index].parent = Some(self.focus);
            let move_data = MoveData {
                ply: self.nodes[self.focus].board.get_fullmove(),
                san: self.nodes[self.focus].board.write_step(step).unwrap(),
                player: self.nodes[self.focus].board.get_active_player(),
            };
            self.nodes[self.focus].sons.push((step, new_index, move_data));
            self.focus = new_index;
            true
        } else {
            false
        }
    }

    fn collect_remaining_nodes(
        &mut self, 
        current: usize, 
        node_to_delete: usize, 
        remaining_nodes: &mut Vec<usize>,
        node_mapping: &mut Vec<Option<usize>>,
    ) {
        if current == node_to_delete {
            return;
        }

        let new_index = remaining_nodes.len();
        remaining_nodes.push(current);
        node_mapping[current] = Some(new_index);

        for (_, son, _) in self.nodes[current].sons.clone() {
            self.collect_remaining_nodes(son, node_to_delete, remaining_nodes, node_mapping);
        }
    }

    pub fn handle_delete_variation(&mut self, e: &DeleteVariationEvent, ew: &mut EventWriter<UpdateBoard<B>>) {
        let current = e.node_id;
        if current == self.root {
            warn!("Try to delete game tree root");
            return;
        }

        let mut remaining_nodes = Vec::new();
        let mut node_mapping = vec![None; self.nodes.len()];

        self.collect_remaining_nodes(self.root, current, &mut remaining_nodes, &mut node_mapping);

        // 创建新的节点向量并更新索引
        let mut new_nodes = Vec::with_capacity(remaining_nodes.len());
        
        // 首先创建所有节点（但sons和parent还未更新）
        for &old_index in &remaining_nodes {
            let mut node = self.nodes[old_index].clone();
            node.sons.clear(); // 清空子节点，稍后重新建立
            node.parent = node.parent.and_then(|p| node_mapping[p]); // 更新父节点索引
            new_nodes.push(node);
        }
        
        // 然后重新建立子节点关系
        for &old_index in &remaining_nodes {
            let new_index = node_mapping[old_index].unwrap();
            for (step, son_old_index, move_data) in &self.nodes[old_index].sons {
                if let Some(son_new_index) = node_mapping[*son_old_index] {
                    new_nodes[new_index].sons.push((*step, son_new_index, move_data.clone()));
                }
            }
        }

        self.nodes = new_nodes;
        self.root = 0;
        // 更新焦点。如果原来的焦点被删除，将焦点移到根并更新棋盘。
        if node_mapping[self.focus].is_none() {
            self.focus = 0;
            ew.write(UpdateBoard::new(self.board()));
        } else {
            self.focus = node_mapping[self.focus].unwrap();
        }
    }

    pub fn handle_move_to_node(&mut self, e: &MoveToNodeEvent, ew: &mut EventWriter<UpdateBoard<B>>) {
        let node_id = e.node_id;
        if node_id >= self.nodes.len() {
            warn!("game_tree: try to move to a node that does not exist.");
            return;
        }
        self.focus = node_id;
        ew.write(UpdateBoard::new(self.board()));
    }

    fn show_context_menu(
        &mut self,
        current: usize,
        response: &egui::Response,
        ew_dv: &mut EventWriter<DeleteVariationEvent>,
    ) {
        egui::Popup::context_menu(response)
            .show(|ui| {
            ui.set_min_width(120.0);
            
            if ui.button("Promote Variation").clicked() {
                if let Some(parent) = self.nodes[current].parent {
                    let mut pos = 0;
                    for (idx, (_, son_id, _)) in self.nodes[parent].sons.iter().enumerate() {
                        if *son_id == current {
                            pos = idx;
                        }
                    }
                    if pos != 0 {
                        self.nodes[parent].sons.swap(0, pos);
                    }
                }
            }
            
            if ui.button("Set as mainline").clicked() {
                let mut cur = current;
                while cur != self.root {
                    if let Some(parent) = self.nodes[cur].parent {
                        let mut pos = 0;
                        for (idx, (_, son_id, _)) in self.nodes[parent].sons.iter().enumerate() {
                            if *son_id == cur {
                                pos = idx;
                            }
                        }
                        if pos != 0 {
                            self.nodes[parent].sons.swap(0, pos);
                        }
                        cur = parent;
                    } else {
                        unreachable!()
                    }
                }
            }
            
            if ui.button("Delete Variation").clicked() {
                ew_dv.write(DeleteVariationEvent::new(current));
            }
            
            if ui.button("Copy PGN").clicked() {
                ui.ctx().copy_text(self.pgn(current));
            }
        });
    }

    fn get_text_width(s: &String, f: &Fonts, font_id: &FontId) -> f32 {
        let mut res: f32 = 0.0;
        for c in s.chars() {
            res += f.glyph_width(font_id, c);
        }
        res
    }

    fn show_labels_horizontal(
        &mut self, 
        ui: &mut egui::Ui, 
        prefix: String, 
        labels: Vec<(String, usize)>,
        ew_mtn: &mut EventWriter<MoveToNodeEvent>,
        ew_dv: &mut EventWriter<DeleteVariationEvent>,
    ) {
        let font_id = egui::FontId::default();
        let mono_font = FontId::monospace(14.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new(prefix.clone()).font(mono_font.clone()));
            for (_, (s, idx)) in labels.iter().enumerate() {
                let new_width = ui.fonts(|f| Self::get_text_width(s, f, &font_id));
                let response = ui.allocate_response(
                    egui::Vec2::new(
                        new_width,
                        ui.text_style_height(&egui::TextStyle::Body),
                    ),
                    egui::Sense::click(), 
                );
                if *idx == self.focus {
                    let rect = response.rect;
                    // 使用默认悬停颜色
                    ui.painter().rect_filled(rect, 2.0, ui.visuals().widgets.hovered.bg_fill);
                }
                if response.hovered() {
                    let rect = response.rect;
                    // 使用默认悬停颜色
                    ui.painter().rect_filled(rect, 2.0, ui.visuals().widgets.hovered.bg_fill);
                }
                ui.painter().text(
                    response.rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    s,
                    font_id.clone(),
                    ui.visuals().text_color(),
                );
                if response.clicked() {
                    ew_mtn.write(MoveToNodeEvent::new(*idx));
                }
                self.show_context_menu(*idx, &response, ew_dv);
            }
        });
    }

    // 展示一些步，自动换行，换行前使用 header_prefix 作为前缀，换行后使用 prefix 作为前缀
    fn show_labels(
        &mut self, 
        ui: &mut egui::Ui, 
        header_prefix: String, 
        prefix: String, 
        labels: Vec<(String, usize)>,
        ew_mtn: &mut EventWriter<MoveToNodeEvent>,
        ew_dv: &mut EventWriter<DeleteVariationEvent>,
    ) {
        let font_id = egui::FontId::default();
        let mono_font = FontId::monospace(14.0);
        let initial_width = ui.available_width();
        let mut width = initial_width;
        width -= ui.fonts(|f| Self::get_text_width(&header_prefix, f, &mono_font));
        let mut last = 0;
        let mut first_line = true;
        for (idx, (s, _)) in labels.iter().enumerate() {
            let new_width = ui.fonts(|f| Self::get_text_width(s, f, &font_id));
            if width < new_width + ui.spacing().item_spacing.x {
                self.show_labels_horizontal(
                    ui, 
                    if first_line { header_prefix.clone() } else { prefix.clone() }, 
                    Vec::from(&labels[last..idx]), 
                    ew_mtn,
                    ew_dv,
                );
                last = idx;
                first_line = false;
                width = initial_width;
                width -= ui.fonts(|f| Self::get_text_width(&prefix, f, &mono_font));
            }
            width -= new_width + ui.spacing().item_spacing.x;
        }
        self.show_labels_horizontal(
            ui, 
            if first_line { header_prefix.clone() } else { prefix.clone() }, 
            Vec::from(&labels[last..]), 
            ew_mtn,
            ew_dv,
        );
    }

    fn dfs_branch(
        &mut self, 
        current: usize, 
        mut labels: Vec<(String, usize)>,
        header_prefix: String,
        prefix: String,
        ui: &mut egui::Ui,
        ew_mtn: &mut EventWriter<MoveToNodeEvent>,
        ew_dv: &mut EventWriter<DeleteVariationEvent>,
    ) {
        let son_num = self.nodes[current].sons.len();
        if son_num > 1 {
            self.show_labels(ui, header_prefix.clone(), prefix.clone(), labels, ew_mtn, ew_dv);

            let new_header = format!("{prefix}{PRE1}");
            let new_pre = format!("{prefix}{PRE3}");
            let last_header = format!("{prefix}{PRE2}");
            let last_pre = format!("{prefix}{PRE4}");

            for i in 0..son_num {
                let (_step, son, move_data) = &self.nodes[current].sons[i];
                let son = *son;
                let san = match move_data.player {
                    PlayerOrder::First => format!("{}.{}", move_data.ply, move_data.san),
                    PlayerOrder::Second => format!("{}...{}", move_data.ply, move_data.san),
                };
                
                self.dfs_branch(
                    son,
                    vec![(san, son)],
                    if i == son_num - 1 { last_header.clone() } else { new_header.clone() },
                    if i == son_num - 1 { last_pre.clone() } else { new_pre.clone() },
                    ui,
                    ew_mtn,
                    ew_dv,
                );
            }
        } else if son_num == 1 {
            let (_step, son, move_data) = &self.nodes[current].sons[0];
            let son = *son;
            let san = match move_data.player {
                PlayerOrder::First => format!("{}.{}", move_data.ply, move_data.san),
                PlayerOrder::Second => format!("{}...{}", move_data.ply, move_data.san),
            };
            self.dfs_branch(
                son, 
                {
                    labels.push((san, son));
                    labels
                },
                header_prefix, 
                prefix, 
                ui, 
                ew_mtn,
                ew_dv);
        } else {
            self.show_labels(ui, header_prefix, prefix, labels, ew_mtn, ew_dv);
        }
    }

    fn dfs_mainline(
        &mut self, 
        current: usize,
        ui: &mut egui::Ui,
        ew_mtn: &mut EventWriter<MoveToNodeEvent>,
        ew_dv: &mut EventWriter<DeleteVariationEvent>,
    ) {
        let son_num = self.nodes[current].sons.len();
        let total_width = ui.available_width();

        if son_num >= 1 {
            // 有支线时先展示支线
            for i in 1..son_num {
                let (_step, son, move_data) = &self.nodes[current].sons[i];
                let son = *son;
                let san = match move_data.player {
                    PlayerOrder::First => format!("{}.{}", move_data.ply, move_data.san),
                    PlayerOrder::Second => format!("{}...{}", move_data.ply, move_data.san),
                };
                
                self.dfs_branch(
                    son,
                    vec![(san, son)],
                    if i == son_num - 1 { String::from(PRE2) } else { String::from(PRE1) },
                    if i == son_num - 1 { String::from(PRE4) } else { String::from(PRE3) },
                    ui,
                    ew_mtn,
                    ew_dv,
                );
            }
            let (_step, son, move_data) = self.nodes[current].sons[0].clone();
            match move_data.player {
                PlayerOrder::First => {
                    ui.horizontal(|ui| {
                        ui.add_sized([total_width * 0.15, 0.0], egui::Label::new(move_data.ply.to_string()));
                        let response = ui.add_sized(
                            [total_width * 0.40, 0.0],
                            egui::Label::new(move_data.san.clone()).sense(Sense::click()),
                        );
                        // if response.secondary_clicked() {
                        //     self.context_menu = Some(son);
                        // }
                        if son == self.focus {
                            let rect = response.rect;
                            // 使用默认悬停颜色
                            ui.painter().rect_filled(rect, 2.0, ui.visuals().widgets.hovered.bg_fill);
                        }
                        if response.hovered() {
                            let rect = response.rect;
                            // 使用默认悬停颜色
                            ui.painter().rect_filled(rect, 2.0, ui.visuals().widgets.hovered.bg_fill);
                        }
                        // 如果文字被颜色覆盖，重新绘制文字
                        if son == self.focus || response.hovered() {
                            let rect = response.rect;
                            ui.painter().text(
                                rect.center(),
                                Align2::CENTER_CENTER,
                                move_data.san.clone(),
                                egui::FontId::default(),
                                ui.visuals().text_color(),
                            );
                        }
                        if response.clicked() {
                            ew_mtn.write(MoveToNodeEvent::new(son));
                        }
                        self.show_context_menu(son, &response, ew_dv);
                        ui.add_sized([total_width * 0.40, 0.0], egui::Label::new("..."));
                    });
                },
                PlayerOrder::Second => {
                    ui.horizontal(|ui| {
                        ui.add_sized([total_width * 0.15, 0.0], egui::Label::new(move_data.ply.to_string()));
                        ui.add_sized([total_width * 0.40, 0.0], egui::Label::new("..."));
                        let response = ui.add_sized(
                            [total_width * 0.40, 0.0],
                            egui::Label::new(move_data.san.clone()).sense(Sense::click()),
                        );
                        // if response.secondary_clicked() {
                        //     self.context_menu = Some(son);
                        // }
                        if son == self.focus {
                            let rect = response.rect;
                            // 使用默认悬停颜色
                            ui.painter().rect_filled(rect, 2.0, ui.visuals().widgets.hovered.bg_fill);
                        }
                        if response.hovered() {
                            let rect = response.rect;
                            // 使用默认悬停颜色
                            ui.painter().rect_filled(rect, 2.0, ui.visuals().widgets.hovered.bg_fill);
                        }
                        // 如果文字被颜色覆盖，重新绘制文字
                        if son == self.focus || response.hovered() {
                            let rect = response.rect;
                            ui.painter().text(
                                rect.center(),
                                Align2::CENTER_CENTER,
                                move_data.san.clone(),
                                egui::FontId::default(),
                                ui.visuals().text_color(),
                            );
                        }
                        if response.clicked() {
                            ew_mtn.write(MoveToNodeEvent::new(son));
                        }
                        self.show_context_menu(son, &response, ew_dv);
                    });
                },
            }
            self.dfs_mainline(son, ui, ew_mtn, ew_dv);
        }
    }

    pub fn display_egui(
        &mut self, 
        ui: &mut egui::Ui,
        ew_mtn: &mut EventWriter<MoveToNodeEvent>,
        ew_dv: &mut EventWriter<DeleteVariationEvent>,
    ) {
        self.dfs_mainline(self.root, ui, ew_mtn, ew_dv);
    }
}

const PRE1: &str = "├─";
const PRE2: &str = "└─";
const PRE3: &str = "| ";
const PRE4: &str = "  ";