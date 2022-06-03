extern crate smallvec;
extern crate colored;
extern crate isahc;
extern crate serde;
extern crate serde_json;

mod bitset;
mod board;
mod dictionary;
mod point;
mod web;
//mod solver;

pub use bitset::Bitset;
pub use board::Board;
pub use point::{Point,PointSet};
pub use web::Puzzle;

fn main() {
    for i in 1..=127 {
        println!("Testing puzzle id {}", i);
        let mut board: Board = web::Puzzle::from_id(i).unwrap().into();
        println!("Loaded board");
        let solutions = board.enumerate_solutions();
        println!("Found {} solutions", solutions.len());
        if solutions.len() > 1 {
            for solution in solutions {
                println!("{}", solution);
            }
        }
        println!();
    }
}

