use crate::grid::Board;

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

enum Turn {
    WhiteNeutron,
    WhitePiece,
    BlackNeutron,
    BlackPiece
}

struct Game {
    board : Board,
    turn: Turn
}

impl Game {
    pub fn new(size: usize) -> Result<Game, String> {
        let board = Board::new(size);
        match board {
            Err(message) => {
                return Err(message);
            }
            Ok(board) => {
                return Ok(Game{board, turn: Turn::WhitePiece});
            }
        }
    }

    pub fn move_piece(&mut self, piece : (usize, usize), direction : Direction) -> Result<(),String> {
        
    }
}