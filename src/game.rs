mod game_io;

use crate::grid::Board;
use crate::enums::{Cell, Winner, Direction, GameError, BoardError};
use game_io::GameIO;

type Pos = (i8, i8);

impl From<Direction> for Pos {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (0, -1),
            Direction::UpRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, 1),
            Direction::Down => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (-1, -1),
        }
    }
}


/// Whose turn it is to play and what piece do they have to move
enum Turn {
    WhiteNeutron,
    WhitePiece,
    BlackNeutron,
    BlackPiece
}

pub struct Game {
    board : Board,
    turn: Turn,
}

impl Game {

    /// Create a new `Game` of Neutron with a given size
    /// It is White to play and they have to move a piece
    /// ## Error
    /// Return `Err` if the `size` is even
    pub fn new(size: usize) -> Result<Game, BoardError> {
        let board = Board::new(size);
        match board {
            Err(e) => {
                return Err(e);
            }
            Ok(board) => {
                return Ok(Game{board, turn: Turn::WhitePiece});
            }
        }
    }

    pub fn show_board(&self) {
        println!("{}", self.board);
    }

    pub fn game_state(&self) -> Option<Winner> {
        match self.board.game_state() {
            Some(Winner::NeutronIsBlocked) => match self.turn {
                Turn::BlackNeutron => Some(Winner::White),
                Turn::WhiteNeutron => Some(Winner::Black),
                _ => None
            },
            Some(w) => Some(w),
            None => None
        }
    }

    /// Check if the move is inside the `Board`
    fn is_move_valid(&self, piece: (i8, i8)) -> bool {
        let x = piece.0 as usize;
        let y = piece.1 as usize;
        return x < self.board.get_size() && y < self.board.get_size() && *self.board.get_unchecked((x,y)) == Cell::Empty;
    }

    /// Evaluate the farthest position that a piece can travel in a given `Direction`
    fn get_max_pos(&self, piece : (usize, usize), direction : (i8, i8)) -> (usize, usize) {
        let mut x = piece.0 as i8;
        let mut y = piece.1 as i8;

        while self.is_move_valid((x+direction.0, y+direction.1)) {
            x += direction.0;
            y += direction.1;
        }
        
        return (x as usize, y as usize);
    }

    /// Move a piece in a given `Direction`
    pub fn move_piece(&mut self, cell_pos : (usize, usize), direction : Direction) -> Result<(),GameError> {
        let cell = *self.board.get_unchecked(cell_pos);
        if cell == Cell::Empty {
            return Err(GameError::TryToMoveEmptyCell);
        } else {
            let new_pos = self.get_max_pos(cell_pos, direction.into());
            if new_pos == cell_pos {
                return Err(GameError::DidNotMoved);
            }
            self.board.set(cell_pos, Cell::Empty);
            self.board.set(new_pos, cell);
            Ok(())
        }
    }

    /// Make the player move the Neutron with respect to the rules of the game
    fn move_neutron(&mut self) {
        let direction = GameIO::ask_user_for_direction();
        
    }

    ///
    fn move_player(&mut self, turn : Turn) {

    }

    


    /// Game loop with game logic
    pub fn play(&mut self) -> Winner {
        let mut winner = None;
        while winner == None {

            self.show_board();
            match self.turn {
                Turn::WhiteNeutron => {
                    self.move_neutron();
                    self.turn = Turn::WhitePiece;
                },
                Turn::WhitePiece => {
                    self.move_player(Turn::WhitePiece); // Vérif qu'une pièce BLANCHE a été bougé
                    self.turn = Turn::BlackNeutron;
                },
                Turn::BlackNeutron => {
                    self.move_neutron();
                    self.turn = Turn::BlackPiece;
                },
                Turn::BlackPiece => {
                    self.move_player(Turn::BlackPiece); // Vérif qu'une pièce NOIRE a été bougé
                    self.turn = Turn::WhiteNeutron;
                }
            }
            winner = self.game_state();
        }

        winner.unwrap()
    }
}