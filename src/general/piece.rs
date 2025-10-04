#[derive(Clone, Copy, PartialEq)]
pub enum PlayerOrder {
    First,
    Second,
}

impl PlayerOrder {
    pub fn flip(&self) -> Self {
        match self {
            PlayerOrder::First => PlayerOrder::Second,
            PlayerOrder::Second => PlayerOrder::First,
        }
    }
}