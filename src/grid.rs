use core::fmt;
use std::fmt::write;
use colored::Colorize;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    White,
    Black,
    Neutron,
    Empty
}

pub enum Winner {
    White,
    Black,
    LastPlayed
}

type Pos = (usize, usize);

pub struct Board {
    grid : Vec<Vec<Cell>>,
    size : usize
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();
        let mut iter = self.grid.iter().flatten();
        for _ in 0..(self.size*self.size) {
            debug_list.entries(iter.next());
        }
        debug_list.finish()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter() {
            for cell in row {
                let symbol = match cell {
                    Cell::White => "○".white(),
                    Cell::Black => "●".black(),
                    Cell::Neutron => "◎".red(),
                    Cell::Empty => "·".dimmed(),
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new(size: usize) -> Result<Board, String> {
        if size % 2 != 1 {
            return Err(String::from("Invalid size"));
        }
        let mut grid = Vec::with_capacity(size);
        for y in 0..size {
            let mut row;
            if y == 0 {
                row = vec![Cell::Black; size];
            }
            else if y == size - 1 {
                row = vec![Cell::White; size];
            }
            else {
                row = vec![Cell::Empty; size];
            }
            if (size-1)/2 == y {
                row[(size-1)/2] = Cell::Neutron;
            }
            grid.push(row);
        }

        Ok(Board{grid,size})
    }

    pub fn new_classic() -> Board {
        return Board::new(5).unwrap();
    }

    pub fn new_big() -> Board {
        return Board::new(7).unwrap();
    }

    pub fn is_valid(&self) -> bool {
        let mut count_w: usize = 0;
        let mut count_b: usize = 0;
        let mut is_neutron: bool = false;
        for cell in self.grid.iter().flatten() {
            match *cell {
                Cell::White => count_w += 1,
                Cell::Black => count_b += 1,
                Cell::Neutron => {
                    if is_neutron {
                        return false;
                    }
                    is_neutron = true;
                },
                Cell::Empty => {},
            }
        }

        if count_b == self.size && count_w == self.size && is_neutron {
            return true;
        }
        return false;
    }

    pub fn is_neutron_blocked(&self) -> bool {
        let mut pos = (0,0);
        for (c_pos,&cell) in self.grid.iter().flatten().enumerate() {
            if cell == Cell::Neutron {
                pos = ((c_pos)/self.size, (c_pos)%self.size);
                break;
            }
        }
        for dy in 0..2_usize {
            if (dy == 0 && pos.1 == 0) || (dy == 2 && pos.1 == self.size -1) {
                continue;
            }
            for dx in 0..2_usize {
                if (dx == 0 && pos.0 == 0) || (dx == 2 && pos.0 == self.size -1) {
                    continue;
                }

                if !(dx == dy && dx == 1) {
                    let cell = self.grid[pos.1 + dy -1][pos.0 + dx -1];
                    if cell == Cell::Empty {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    pub fn state_of_the_game(&self) -> Option<Winner> {
        if self.grid[0].iter().filter(|&&cell| cell == Cell::Neutron).count() != 0 {
            return Some(Winner::Black);
        }

        if self.grid[self.size -1].iter().filter(|&&cell| cell == Cell::Neutron).count() != 0 {
            return Some(Winner::White);
        }

        if self.is_neutron_blocked() {
            return Some(Winner::LastPlayed);
        }
        
        None
    }

    pub fn get_unchecked(&self, cell : Pos) -> &Cell {
        let (x,y) = cell;
        if x >= self.size || y >= self.size {
            panic!("Index out of bound : got {:?} but shape is {:?} !", cell, self.size);
        }
        &self.grid[y][x]
    }

    pub fn get(&self, cell : Pos) -> Option<&Cell> {
        let (x,y) = cell;
        if x >= self.size || y >= self.size {
            return None;
        }
        Some(&self.grid[y][x])
    }

    pub fn set(&mut self, cell : Pos, piece : Cell) {
        self.grid[cell.1][cell.0] = piece;
    }
}