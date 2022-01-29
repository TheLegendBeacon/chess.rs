#[allow(unused_imports)]
#[allow(unused_variables)]
use super::grid::{Coordinate, Grid};
use super::piece::{PieceColour};
use super::piece::PieceKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn to_coordinate_modifier(&self, is_white: bool) -> (isize, isize) {
        let coordinate_modifier = match self {
            &Direction::N => (0, 1),
            &Direction::NE => (1, 1),
            &Direction::E => (1, 0),
            &Direction::SE => (1, -1),
            &Direction::S => (0, -1),
            &Direction::SW => (-1, -1),
            &Direction::W => (-1, 0),
            &Direction::NW => (-1, 1),
        };

        let colour_modifier = match is_white {
            true => 1,
            false => -1
        };

        (coordinate_modifier.0 * colour_modifier, coordinate_modifier.1 * colour_modifier)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal(Coordinate),
    Attack(Coordinate),
    Castling(Coordinate),
    Promotion(Coordinate)
}

impl MoveType {
    fn get_coord(&self) -> &Coordinate {
        match self {
          MoveType::Normal(c) => c,
          MoveType::Attack(c) => c,
          MoveType::Castling(c) => c,
          MoveType::Promotion(c) => c,
        }
      }
}

// VALIDATION ONLY

impl PieceKind {
    pub fn move_number(&self) -> u8 {
        /*
        0 - 0 moves
        1 - 1 move
        2 - infinite moves
        */
        match self {
            PieceKind::Blank => 0,
            PieceKind::Pawn => 1,
            PieceKind::Knight => 1,
            PieceKind::King => 1,
            _ => 2
        }
    }

    fn get_directions(&self) -> (Vec<Direction>, Vec<Direction>) {
        let item = match self {
            PieceKind::Blank => vec![vec![]; 2],
            PieceKind::Pawn => vec![vec![Direction::N], vec![Direction::NE, Direction::NW]],
            PieceKind::Bishop => {
                vec![vec![Direction::NE, Direction::NW, Direction::SE, Direction::SW]; 2]
            }
            PieceKind::Rook => {
                vec![vec![Direction::N, Direction::W, Direction::E, Direction::S]; 2]
            }
            PieceKind::King => vec![
                vec![
                    Direction::N,
                    Direction::W,
                    Direction::E,
                    Direction::S,
                    Direction::NE,
                    Direction::NW,
                    Direction::SE,
                    Direction::SW
                ]; 2],
            PieceKind::Queen => vec![
                vec![
                    Direction::N,
                    Direction::W,
                    Direction::E,
                    Direction::S,
                    Direction::NE,
                    Direction::NW,
                    Direction::SE,
                    Direction::SW
                ];
                2
            ],
            PieceKind::Knight => vec![vec![Direction::N]; 2],
        };
        (item[0].to_owned(), item[1].to_owned())
    }
}


fn generate_possible_moves(start_coord: &Coordinate, grid: &Grid) -> Result<Vec<MoveType>, ()> {
    let mut moves = Vec::new();
    let piece = grid.get_piece(&start_coord)?;
    
    if piece.kind != PieceKind::Knight {
        let (move_directions, attack_directions) = piece.kind.get_directions();

        for direction in move_directions {
            let mut move_counter = 1;
            let max_moves = match piece.kind.move_number() {
                0 => 0,
                1 => 1,
                _ => 8
            };
            let coordinate_modifier = direction.to_coordinate_modifier(piece.colour == PieceColour::White);

            loop {
                if move_counter > max_moves {
                    break;
                }

                let new_coord = Coordinate(
                    ((start_coord.0 as isize) + coordinate_modifier.0*move_counter as isize) as u8 as char,
                    ((start_coord.1 as isize) + coordinate_modifier.1*move_counter as isize) as usize
                );

                if grid.is_valid_coordinate(&new_coord) {
                    if grid.get_piece(&new_coord)?.kind == PieceKind::Blank {
                        moves.push(MoveType::Normal(new_coord));
                    } else {
                        break;
                    }
                } else {
                    break;
                }

                move_counter += 1;
                continue;
            }
        }
    }
    return Ok(moves);
}

impl Grid {
    pub fn validate_move(
        &self,
        start_coord: &Coordinate,
        end_coord: &Coordinate,
    ) -> Result<(), ()> {
        for item in generate_possible_moves(start_coord, self)? {
            if item.get_coord() == end_coord {
                return Ok(());
            }
        }
    Err(())
    }
}