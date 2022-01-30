/*
[TODO]
1. Interface: Far far away; low priority.
*/

mod fen_parser;
mod grid;
mod piece;
mod validation;

use grid::{Coordinate, Grid};

fn main() {
    let mut grid = Grid::new();
    println!("{:?}", grid);
}
