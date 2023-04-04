
pub enum GameState {
    Standby,
    InGame,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_opposite(&self, direction: Self) -> bool {
        let dir = self.to_owned();
        match direction {
            Self::Down => dir == Self::Up,
            Self::Up => dir == Self::Down,
            Self::Right => dir == Self::Left,
            Self::Left => dir == Self::Right,
        }
    }
}