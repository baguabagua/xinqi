#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerSet {
    None,
    First,
    Second,
    All,
}

impl PlayerSet {
    pub fn include(&self, player: PlayerOrder) -> bool {
        match self {
            PlayerSet::None => false,
            PlayerSet::First => player == PlayerOrder::First,
            PlayerSet::Second => player == PlayerOrder::Second,
            PlayerSet::All => true,
        }
    }
}