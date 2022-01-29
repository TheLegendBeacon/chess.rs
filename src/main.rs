/*
[TODO]
1. Interface: Far far away; low priority.
*/

mod fen_parser;

#[allow(dead_code)]
mod grid;
mod piece;

#[allow(dead_code)]
#[allow(unused_variables)]
mod validation;

use grid::{Coordinate, Grid};

fn main() {
    let mut grid = Grid::new();
    println!("{:?}", grid);
