#[allow(unused_imports)]
#[allow(unused_variables)]
use super::grid::{Coordinate, Grid};
use super::piece::PieceColour;
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
            false => -1,
        };

        (
            coordinate_modifier.0 * colour_modifier,
            coordinate_modifier.1 * colour_modifier,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal(Coordinate),
    Attack(Coordinate),
    Castling(Coordinate),
    Promotion(Coordinate),
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
            _ => 2,
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
                ];
                2
            ],
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
    
        let max_moves = match piece.kind.move_number() {
            0 => 0,
            1 => 1,
            _ => 8,
        };

        for direction in move_directions {
            let mut move_counter = 1;

            let coordinate_modifier =
                direction.to_coordinate_modifier(piece.colour == PieceColour::White);

            loop {
                if move_counter > max_moves {
                    break;
                }

                let new_coord = Coordinate(
                    ((start_coord.0 as isize) + coordinate_modifier.0 * move_counter as isize) as u8
                        as char,
                    ((start_coord.1 as isize) + coordinate_modifier.1 * move_counter as isize)
                        as usize,
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
    
        for direction in attack_directions {
            let mut move_counter = 1;

            let coordinate_modifier =
                direction.to_coordinate_modifier(piece.colour == PieceColour::White);

            loop {
                if move_counter > max_moves {
                    break;
                } 

                let new_coord = Coordinate(
                    ((start_coord.0 as isize) + coordinate_modifier.0 * move_counter as isize) as u8
                        as char,
                    ((start_coord.1 as isize) + coordinate_modifier.1 * move_counter as isize)
                        as usize,
                );

                if grid.is_valid_coordinate(&new_coord) {

                    if grid.get_piece(&new_coord)?.kind != PieceKind::Blank && grid.get_piece(&new_coord)?.colour != piece.colour {
                        moves.push(MoveType::Attack(new_coord));
                        break;
                    }
                }
                move_counter += 1;
            }
        }
    } else {
        let possible_modifications = [
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1)
            
        ];

        let mut possible_coordinates: Vec<Coordinate> = Vec::new();
        for item in possible_modifications {
            let new_coord = Coordinate(
                ((start_coord.0 as isize) + item.0 as isize) as u8
                    as char,
                ((start_coord.1 as isize) + item.1 as isize)
                    as usize,
            );

            if grid.is_valid_coordinate(&new_coord) {
                possible_coordinates.push(new_coord);
            }
        }
        for item in possible_coordinates {
            if grid.get_piece(&item)?.kind == PieceKind::Blank {
                moves.push(MoveType::Normal(item));
            } else {
                moves.push(MoveType::Attack(item));
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

#[allow(dead_code)]
#[allow(unused_imports)]
mod tests {
    use super::super::grid::{Coordinate, Grid};

    #[test]
    fn basic_normal_move() {
        let mut grid = Grid::new();

        grid.move_piece(&Coordinate('C', 2), &Coordinate('C', 3))
            .unwrap(); // Pawn
        grid.move_piece(&Coordinate('D', 1), &Coordinate('C', 2))
            .unwrap(); // Queen
        grid.move_piece(&Coordinate('E', 1), &Coordinate('D', 1))
            .unwrap(); // King

        grid.move_piece(&Coordinate('D', 2), &Coordinate('D', 3))
            .unwrap(); // Move the pawn out of the way
        grid.move_piece(&Coordinate('A', 2), &Coordinate('A', 3))
            .unwrap(); // Move the pawn out of the way

        grid.move_piece(&Coordinate('C', 1), &Coordinate('G', 5))
            .unwrap(); // Bishop
        grid.move_piece(&Coordinate('A', 1), &Coordinate('A', 2))
            .unwrap(); // Rook
        grid.move_piece(&Coordinate('B', 1), &Coordinate('D', 2))
            .unwrap(); // Knight
    }

    #[test]
    #[should_panic]
    fn basic_validation_panic() {
        let mut grid = Grid::new();

        grid.move_piece(&Coordinate('A', 69), &Coordinate('B', 420))
            .unwrap();
    }

    #[test]
    fn basic_attacking_move() {
        let mut grid = Grid::new();

        grid.move_piece(&Coordinate('E', 2), &Coordinate('E', 3)).unwrap();
        grid.move_piece(&Coordinate('D', 1), &Coordinate('H', 5)).unwrap();
        grid.move_piece(&Coordinate('H', 5), &Coordinate('F', 7)).unwrap();
    }

    #[test]
    fn basic_black_test() {
        let mut grid = Grid::new();

        grid.move_piece(&Coordinate('E', 7), &Coordinate('E', 6)).unwrap();
    }

    #[test]
    fn pawn_attack_test() {
        let mut grid = Grid::new();
        
        grid.move_piece(&Coordinate('D', 7), &Coordinate('D', 6)).unwrap();
        grid.move_piece(&Coordinate('D', 8), &Coordinate('D', 7)).unwrap();
        grid.move_piece(&Coordinate('D', 7), &Coordinate('H', 3)).unwrap();

        grid.move_piece(&Coordinate('G', 2), &Coordinate('H', 3)).unwrap();

    }
}
