use std::io;

use crate::enums::Direction;
use super::Turn;

use colored::Colorize;
use regex::Regex;

pub struct GameIO;

impl GameIO {

    /// Check if the coordinates have the right syntax (A1, B2, C3, ...)
    pub fn is_coordinates_valid(input: &String) -> bool {
        let input = input.trim().to_uppercase();
        let regex = Regex::new(r"^[A-Z][1-9]\d*$").unwrap();
        
        regex.is_match(&*input)
    }

    /// Check if the direction are numbers between 1 and 8
    pub fn is_direction_valid(input: &String) -> bool {
        let input = input.trim().to_uppercase();
        let regex = Regex::new(r"^[1-8]$").unwrap();

        regex.is_match(&*input)
    }
    // pub fn is_direction_valid(input: String) -> bool { // Tu as peut-être envie d'avoir &String en paramètre
    //     let input = input.trim().to_uppercase();
    //     let regex = Regex::new(r"^[1-8]$").unwrap();
    //     return regex.is_match(&input);
    // }

    /// Return the coordinates of the piece the user wants to move
    // pub fn get_piece_from_input(input: String) -> (usize, usize) { // Ici c'est ok
    //     let input = input.trim().to_uppercase();
    //     let x = input.chars().nth(0).unwrap() as usize - 65;
    //     let y = input[1..].parse::<usize>().unwrap() - 1;
    //     return (x, y);
    // }

    pub fn get_position_from_input(input: &String) -> (usize, usize) {
        let upper_input = input.trim().to_uppercase();
        let mut chars = upper_input.chars();
        let x = chars.next().unwrap() as usize - 65;
        let y = chars.next().unwrap().to_string().parse::<usize>().unwrap() -1;
        (x,y)
    }

    /// Ask the user in the terminal a direction
    pub fn ask_user_for_direction() -> Direction {
        let mut input = String::new();

        while !GameIO::is_direction_valid(&input) {
            println!("> {}", "".green());

            if let Err(_) = io::stdin().read_line(&mut input) {
                continue;
            }
        }

        match input.trim().parse::<u8>().unwrap() {
            1 => return Direction::Up,
            2 => return Direction::UpRight,
            3 => return Direction::Right,
            4 => return Direction::DownRight,
            5 => return Direction::Down,
            6 => return Direction::DownLeft,
            7 => return Direction::Left,
            8 => return Direction::UpLeft,
            _ => unreachable!(),
        }
    }

    pub fn ask_user_for_position() -> (usize,usize) {
        let mut input = String::new();
        
        while !GameIO::is_coordinates_valid(&input) {
            println!("> {}", "".green());

            if let Err(_) = io::stdin().read_line(&mut input) {
                continue;
            }
        }

        GameIO::get_position_from_input(&input)
    }

    pub fn reset_terminal_screen() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn annonce_move(turn : &Turn) {
        match *turn {
            Turn::WhiteNeutron => println!("White to play and move the neutron"),
            Turn::WhitePiece => println!("White to play and move a piece"),
            Turn::BlackNeutron => println!("Black to play and move the neutron"),
            Turn::BlackPiece => println!("Black to play and move a piece")
        }
    }

    pub fn give_input_format_for_position() {
        print!("What piece should move next ? ");
        println!("{} {}", "ex : ".dimmed(), "A3".black());
    }

    pub fn give_input_format_for_direction() {
        println!("Please enter a direction {} :", "1-8".green());
        println!("{}", "[1] Up [2] Up Right [3] Right [4] Down Right".dimmed());
        println!("{}", "[5] Down [6] Down Left [7] Left [8] Up Left".dimmed());
    }
}
