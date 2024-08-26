use core::fmt;
use colored::Colorize;
use crate::enums::{Cell, Winner, BoardError};

type Pos = (usize, usize);

///
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
        for i in 0..self.size {
            let letter = (65+i) as u8 as char;
            write!(f, "{} ", letter)?;
        }
        writeln!(f)?;
        for (i, row) in self.grid.iter().enumerate() {
            for cell in row {
                let symbol = match cell {
                    Cell::White => "○".white(),
                    Cell::Black => "●".black(),
                    Cell::Neutron => "◎".blue(),
                    Cell::Empty => "·".dimmed(),
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f, "{}", i+1)?;
        }
        Ok(())
    }
}

impl Board {
    
    /// Create a new `Board` of given `size`.
    /// ## Error
    /// Return an `Err` if the size is even
    pub fn new(size: usize) -> Result<Board, BoardError> {
        if size % 2 != 1 {
            return Err(BoardError::EvenSize);
        }
        else if size == 1 {
            return Err(BoardError::SizeOfOne);
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

    /// Initialize a new `Board` with the classic size (5x5)
    pub fn new_classic() -> Board {
        return Board::new(5).unwrap();
    }

    /// Initialize a new `Board` with the big size (7x7)
    pub fn new_big() -> Board {
        return Board::new(7).unwrap();
    }

    /// Check the correct number of pieces on the `Board` and if the neutron is present
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

    /// Check if the piece at given `pos` can be moved. Does NOT check if there is
    /// an actual piece
    fn is_piece_blocked(&self, pos : Pos) -> bool {
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

    /// Return the position of the Neutron on the `Board`
    pub fn get_neutron(&self) -> Pos {
        for (c_pos,&cell) in self.grid.iter().flatten().enumerate() {
            if cell == Cell::Neutron {
                return ((c_pos)/self.size, (c_pos)%self.size);
            }
        }
        panic!("WHERE IS THE NEUTRON ON THE BOARD ?!!?") // Never come here
    }

    /// Check if the neutron is blocked and cannot be moved again
    pub fn is_neutron_blocked(&self) -> bool {
        self.is_piece_blocked(self.get_neutron())
        // Si ça fonctionne pas car moved 2 fois ->
        // let pos = self.get_neutron();
        // self.is_piece_blocked(pos)
    }

    /// Check if the game ended and the return the potential `Winner`
    pub fn game_state(&self) -> Option<Winner> {
        if self.grid[0].iter().filter(|&&cell| cell == Cell::Neutron).count() != 0 {
            return Some(Winner::White);
        }

        if self.grid[self.size -1].iter().filter(|&&cell| cell == Cell::Neutron).count() != 0 {
            return Some(Winner::Black);
        }

        if self.is_neutron_blocked() {
            return Some(Winner::NeutronIsBlocked);
        }
        
        None
    }

    /// Access safely to a `Cell`
    pub fn get_unchecked(&self, cell_pos : Pos) -> &Cell {
        let (x,y) = cell_pos;
        if x >= self.size || y >= self.size {
            panic!("Index out of bound : got {:?} but shape is {:?} !", cell_pos, self.size);
        }
        &self.grid[y][x]
    }

    /// Access a `Cell`
    /// ## Panic
    /// Panic if the given position is out of bound
    pub fn get(&self, cell_pos : Pos) -> Option<&Cell> {
        let (x,y) = cell_pos;
        if x >= self.size || y >= self.size {
            return None;
        }
        Some(&self.grid[y][x])
    }

    /// Set the type of `Cell` at a given position
    pub fn set(&mut self, cell_pos : Pos, cell_type : Cell) {
        self.grid[cell_pos.1][cell_pos.0] = cell_type;
    }

    /// Get the `size` of the `Grid`
    pub fn get_size(&self) -> usize {
        self.size
    }
}