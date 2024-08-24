use std::io;

use crate::enums::{Direction, GameIOError};

use colored::Colorize;
use regex::Regex;

pub struct GameIO;

impl GameIO {

    /// Check if the coordinates have the right syntax (A1, B2, C3, ...)
    pub fn is_coordinates_valid(input: String) -> bool {
        let input = input.trim().to_uppercase();
        let regex = Regex::new(r"^[A-Z][1-9]\d*$").unwrap();
        return regex.is_match(&input);
    }

    /// Check if the direction are numbers between 1 and 8
    pub fn is_direction_valid(input: String) -> bool {
        let input = input.trim().to_uppercase();
        let regex = Regex::new(r"^[1-8]$").unwrap();
        return regex.is_match(&input);
    }

    /// Return the coordinates of the piece the user wants to move
    pub fn get_piece_from_input(input: String) -> (usize, usize) {
        let input = input.trim().to_uppercase();
        let x = input.chars().nth(0).unwrap() as usize - 65;
        let y = input[1..].parse::<usize>().unwrap() - 1;
        return (x, y);
    }

    /// Ask the user in the terminal a direction
    pub fn ask_user_for_direction() -> Result<Direction, GameIOError> {
        let mut input = String::new();

        while (!GameIO::is_direction_valid(input)) {
            println!("Please enter a direction {} :", "1-8".green());
            println!("{}", "[1] Up [2] Up Right [3] Right [4] Down Right".dimmed()
            );
            println!("{}", "[5] Down [6] Down Left [7] Left [8] Up Left".dimmed());
            print!("> {}", "".green());

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }

        match input.trim().parse::<u8>().unwrap() {
            1 => Ok(Direction::Up),
            2 => Ok(Direction::UpRight),
            3 => Ok(Direction::Right),
            4 => Ok(Direction::DownRight),
            5 => Ok(Direction::Down),
            6 => Ok(Direction::DownLeft),
            7 => Ok(Direction::Left),
            8 => Ok(Direction::UpLeft),
            _ => Err(GameIOError::InvalidDirection),
        }
    }
}
