use bevy::prelude::*;

pub enum Shape {
    Circle { center: Vec2, radius: f32 },
    Rect { rect: Rect },
}

impl Shape {
    pub fn contain(&self, point: Vec2) -> bool {
        match self {
            Shape::Circle { center, radius } => {
                (point - center).length() <= *radius
            },
            Shape::Rect { rect } => {
                rect.contains(point)
            },
        }
    }
    pub fn center(&self) -> Vec2 {
        match self {
            Shape::Circle { center, radius: _ } => *center,
            Shape::Rect { rect } => rect.center(),
        }
    }
}

#[derive(Component)]
pub struct CellCom {
    pub shape: Shape,
    pub clickable: bool,
    pub dragable: bool,
    pub upper_piece: Entity, // 上方棋子应当有 Transform 属性
}
