use crate::grid::Board;
use crate::game::Game;
use crate::enums::{Direction,Winner};

#[test]
fn classic_board_valid() {
    let board = Board::new_classic();
    println!("{}", board);
    assert!(board.is_valid());
}

#[test]
fn white_wins() -> Result<(),String> {
    let mut game = Game::new(5).unwrap();
    game.move_piece((0,0), Direction::Down);
    assert!(game.game_state() == None);
    game.move_piece((2,2), Direction::UpLeft);
    game.show_board();
    assert_eq!(game.game_state(), Some(Winner::White));
    Ok(())
}

#[test]
fn black_wins() -> Result<(),String> {
    let mut game = Game::new(5).unwrap();
    game.move_piece((4,4), Direction::Up);
    assert!(game.game_state() == None);
    game.move_piece((2,2), Direction::DownRight);
    game.show_board();
    assert_eq!(game.game_state(), Some(Winner::Black));
    Ok(())
}

#[test]
fn neutron_blocked_win() -> Result<(),String> {
    let mut game = Game::new(5).unwrap();
    game.move_piece((0,0), Direction::Down);
    game.move_piece((2,2), Direction::UpLeft);

    game.show_board();
    assert_eq!(game.game_state(), Some(Winner::White));
    Ok(())
}

#[test]
fn char_test() {
    print!("{}", 65 as char);
}

#[test]
#[should_panic(expected = "RAAAAH")]
fn it_dont_work() {
    panic!("RAAAAH");
}
