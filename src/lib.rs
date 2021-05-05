#[derive(Debug, Clone)]
pub struct ChessPosition {
    row: i8,
    col: i8
}

#[derive(Debug, Clone)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(row: i8, col: i8) -> Option<ChessPosition> {
        match (row,col) {
            (0..=7,0..=7) => Some(ChessPosition{row,col}),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dx = self.position.row - other.position.row;
        let dy = self.position.col - other.position.col;
        dx == 0 || dy == 0 || dx.abs() == dy.abs()
    }
}
