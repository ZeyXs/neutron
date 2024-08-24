

/// The 8 possible directions a piece can move
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

/// A `Cell` of the `Board`
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    White,
    Black,
    Neutron,
    Empty
}

/// The `Winner` of the game
#[derive(PartialEq, Debug)]
pub enum Winner {
    White,
    Black,
    NeutronIsBlocked
}

/// Errors at game creation
#[derive(Debug)]
pub enum BoardError {
    SizeOfOne,
    EvenSize
}

/// Possible errors while playing the game
#[derive(Debug)]
pub enum GameError {
    TryToMoveEmptyCell,
    DidNotMoved,
    Invalid
}

/// Errors in Game IO
#[derive(Debug)]
pub enum GameIOError {
    InvalidDirection,
}