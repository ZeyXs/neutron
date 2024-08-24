use crate::grid::Board;

#[test]
fn classic_board_valid() {
    let board = Board::new_classic();
    println!("{}", board);
    assert!(board.is_valid());
}

#[test]
#[should_panic(expected = "RAAAAH")]
fn it_dont_work() {
    panic!("RAAAAH");
}
