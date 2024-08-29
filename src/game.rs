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
#[derive(PartialEq)]
pub enum Turn {
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

    /// Preset of 5x5
    pub fn new_classic() -> Game {
        return Game { board: Board::new_classic(), turn: Turn::WhitePiece }
    }

    /// Preset of 7x7
    pub fn new_big() -> Game {
        return Game { board: Board::new_big(), turn: Turn::WhitePiece }
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
            other => other
        }
    }

    /// Check if the move is inside the `Board`
    fn is_move_valid(&self, piece: (i8, i8)) -> bool {
        let (x,y) = piece;
        if x < 0 || y < 0 {
            return false;
        }
        let (x,y) = (x as usize, y as usize);
        return x < self.board.get_size() && y < self.board.get_size() && *self.board.get_unchecked((x,y)) == Cell::Empty;
    }

    /// Evaluate the farthest position that a piece can travel in a given `Direction`
    fn get_max_pos(&self, piece : (usize, usize), direction : Direction) -> (usize, usize) {
        let (mut x, mut y) = (piece.0 as i8, piece.1 as i8);
        let direction : (i8,i8) = direction.into();

        while self.is_move_valid((x+direction.0, y+direction.1)) {
            x += direction.0;
            y += direction.1;
        }
        
        return (x as usize, y as usize);
    }

    /// Move a piece in a given `Direction`
    /// ## Panic
    /// Panics if `cell_pos` is NOT in the board of the game
    /// ## Error
    /// Return a `TryToMoveEmptyCell` error if `cell_pos` is an empty cell.
    /// Return a `DidNotMoved` error the board is the same as before after the move
    pub fn move_piece(&mut self, cell_pos : (usize, usize), direction : Direction) -> Result<(),GameError> {
        let cell = *self.board.get_unchecked(cell_pos);
        if cell == Cell::Empty {
            return Err(GameError::TryToMoveEmptyCell);
        } else {
            let new_pos = self.get_max_pos(cell_pos, direction);
            if new_pos == cell_pos {
                return Err(GameError::DidNotMoved);
            }
            self.board.set(cell_pos, Cell::Empty);
            self.board.set(new_pos, cell);
            Ok(())
        }
    }

    /// Make the player move the Neutron with respect to the rules of the game
    fn player_turn_neutron(&mut self) {
        let pos = self.board.get_neutron();
        GameIO::give_input_format_for_direction();
        loop {
            let direction = GameIO::ask_user_for_direction();
            match self.move_piece(pos, direction) {
                Ok(()) => return,
                Err(GameError::TryToMoveEmptyCell) => unreachable!(),
                Err(GameError::DidNotMoved) => continue,
            }
        }
    }

    /// Make the player, who's turn it is, move one of their piece with respect to the rules
    fn player_turn_piece(&mut self) {
        let mut piece_pos : (usize, usize);
        GameIO::give_input_format_for_position();
        loop {
            let pos = GameIO::ask_user_for_position();
            match self.board.get(pos) { // Is the position in the board ?

                None => continue,
                Some(cell) => {

                    match *cell { // Is it player's piece ?
                        Cell::White => { // Wrong color : black try to move white piece
                            if self.turn == Turn::BlackPiece {
                                continue;
                            }
                        },
                        Cell::Black => { // Wrong color : white try to move black piece
                            if self.turn == Turn::WhitePiece {
                                continue;
                            }
                        },
                        _ => continue, // Not a piece
                    }

                    piece_pos = pos;
                    break;
                }
            }
        }

        GameIO::give_input_format_for_direction();
        loop {
            let direction = GameIO::ask_user_for_direction();
            match self.move_piece(piece_pos, direction) {
                Ok(_) => {
                    return;
                }
                Err(GameError::TryToMoveEmptyCell) => unreachable!(),
                Err(GameError::DidNotMoved) => continue,
            }
        }
    }

    


    /// Game loop with game logic
    pub fn play(&mut self) -> Winner {
        let mut winner = None;
        while winner == None {
            
            GameIO::reset_terminal_screen();
            self.show_board();
            GameIO::annonce_move(&self.turn);
            match self.turn {
                Turn::WhiteNeutron => {
                    self.player_turn_neutron();
                    self.turn = Turn::WhitePiece;
                },
                Turn::WhitePiece => {
                    self.player_turn_piece();
                    self.turn = Turn::BlackNeutron;
                },
                Turn::BlackNeutron => {
                    self.player_turn_neutron();
                    self.turn = Turn::BlackPiece;
                },
                Turn::BlackPiece => {
                    self.player_turn_piece();
                    self.turn = Turn::WhiteNeutron;
                }
            }
            winner = self.game_state();
        }

        GameIO::reset_terminal_screen();
        self.show_board();
        winner.unwrap()
    }
}