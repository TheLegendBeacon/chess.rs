
// TODO
/*
FEN-1: Expand the FEN parser and allow it to take up more than just the position inputs. LOW priority. Depends on Grid-2.
*/


use super::grid::Grid;
use super::piece::{Piece, PieceColour, PieceKind};

fn from_char(item: char) -> Result<Vec<Piece>, ()> {
    if item.is_digit(10) && item.to_digit(10) <= Some(8) {
        return Ok(vec![
            Piece {
                colour: PieceColour::White,
                kind: PieceKind::Blank,
            };
            item.to_digit(10).unwrap() as usize
        ]);
    } else {
        let (colour, kind) = match item {
            'p' => (PieceColour::Black, PieceKind::Pawn),
            'r' => (PieceColour::Black, PieceKind::Rook),
            'n' => (PieceColour::Black, PieceKind::Knight),
            'b' => (PieceColour::Black, PieceKind::Bishop),
            'q' => (PieceColour::Black, PieceKind::Queen),
            'k' => (PieceColour::Black, PieceKind::King),
            'P' => (PieceColour::White, PieceKind::Pawn),
            'R' => (PieceColour::White, PieceKind::Rook),
            'N' => (PieceColour::White, PieceKind::Knight),
            'B' => (PieceColour::White, PieceKind::Bishop),
            'Q' => (PieceColour::White, PieceKind::Queen),
            'K' => (PieceColour::White, PieceKind::King),
            _ => return Err(()),
        };

        return Ok(vec![Piece { colour, kind }]);
    }
}

pub fn fen_parser(inputs: &str) -> Result<Grid, ()> {
    let items = inputs
        .split("/")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut grid: Vec<Vec<Piece>> = Vec::new();
    for line in items {
        let mut linevec = Vec::new();

        for item in line {
            let pieces = from_char(item);

            match pieces {
                Ok(piece) => {
                    linevec.extend(piece);
                }
                Err(_) => {
                    return Err(());
                }
            }
        }
        grid.push(linevec);
    }
    Ok(Grid::from_attrs(grid, ((true, true), (true, true))))
}
