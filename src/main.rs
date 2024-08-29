mod enums;
mod grid;
mod game;
#[cfg(test)]
mod tests;

use game::Game;

fn main() {
    let mut game = Game::new_classic();
    let winner = game.play();
    println!("{:?}",winner);
}
