// TODO
/*
1. Make the debug for Piece derived. Depends on grid-2. LOW priority.
*/

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColour {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Blank,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub colour: PieceColour,
    pub kind: PieceKind,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.colour == PieceColour::White {
            match self.kind {
                PieceKind::Blank => f.write_str(" "),
                PieceKind::Pawn => f.write_str("♙"),
                PieceKind::Rook => f.write_str("♖"),
                PieceKind::Bishop => f.write_str("♗"),
                PieceKind::Knight => f.write_str("♘"),
                PieceKind::Queen => f.write_str("♕"),
                PieceKind::King => f.write_str("♔"),
            }
        } else {
            match self.kind {
                PieceKind::Blank => f.write_str(" "),
                PieceKind::Pawn => f.write_str("♟"),
                PieceKind::Rook => f.write_str("♜"),
                PieceKind::Bishop => f.write_str("♝"),
                PieceKind::Knight => f.write_str("♞"),
                PieceKind::Queen => f.write_str("♛"),
                PieceKind::King => f.write_str("♚"),
            }
        }
    }
}

// [TODO]: CHANGE THIS
impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{}", self))
    }
}
