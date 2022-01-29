// TODO
/*
`1. Finish move validation. (Validation.rs) HIGH priority.
 2. Make a good display implementation for Grid. Medium priority.
 3. Add the move counter and the half-move counter. LOW priority.
*/

use super::piece::{Piece, PieceColour, PieceKind};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate(pub char, pub usize);

pub struct Grid {
    grid: Vec<Vec<Piece>>,
    can_castle: ((bool, bool), (bool, bool)),
}

// Piece getter and setter
impl Grid {

    // Creates a new chessboard with preset chess pieces.
    pub fn new() -> Grid {
        super::fen_parser::fen_parser("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap()
    }

    // Creates a grid struct from given attributes.
    pub fn from_attrs(grid: Vec<Vec<Piece>>, can_castle: ((bool, bool), (bool, bool))) -> Grid {
        Grid { grid, can_castle }
    }

    // Gets a piece using usual notation of ((a-h), (1-8))
    pub fn get_piece(&self, coords: &Coordinate) -> Result<&Piece, ()> {
        let letter = match coords.0 as u8 {
            65..=90 => coords.0.to_ascii_lowercase(),
            97..=122 => coords.0,
            _ => return Err(()),
        };

        if 0 < coords.1
            && coords.1 <= 8
            && 0 < (letter as usize - 96)
            && (letter as usize - 96) <= 8
        {
            Ok(&(self.grid[8 - (coords.1)][letter as usize - 97]))
        } else {
            Err(())
        }
    }

    // Sets a piece to a given position. Again, using usual chess notation.
    fn set_piece(&mut self, coords: &Coordinate, piece: Piece) -> Result<(), ()> {
        let letter = coords.0.to_ascii_lowercase();
        if 0 < coords.1
            && coords.1 <= 8
            && (letter as usize - 96) > 0
            && (letter as usize - 96) <= 8
        {
            self.grid[8 - (coords.1)][letter as usize - 97] = piece;
            Ok(())
        } else {
            Err(())
        }
    }
}

// Move maker
impl Grid {

    // validation code in validation.rs - this moves a piece and handles the taking of pieces.
    pub fn move_piece(
        &mut self,
        start_coord: &Coordinate,
        end_coord: &Coordinate,
    ) -> Result<(), ()> {

        self.validate_move(start_coord, end_coord)?; // Validation.rs

        let piece_to_move = self.get_piece(start_coord).unwrap().to_owned();
        self.set_piece(end_coord, piece_to_move)?;

        self.set_piece(
            start_coord,
            Piece {
                kind: PieceKind::Blank,
                colour: PieceColour::White,
            },
        )?;
        Ok(())
    }
}

// [TODO]: Make fmt::Display format for this
impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.grid.iter() {
            f.write_str(format!("{:?}\n", line).as_str())?;
        }
        Ok(())
    }
}
