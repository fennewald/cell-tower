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

use std::io;
use std::time::{Duration, Instant};

pub use bitset::Bitset;
pub use board::Board;
pub use point::{Point,PointSet};
pub use web::Puzzle;

fn main() {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop(); // Remove endline
        if let Ok(id) = buffer.parse::<usize>() {
            println!("Loading puzzle {}", id);
            let mut board: Board = web::Puzzle::from_id(id)
                .unwrap()
                .into();
            println!("Loaded");
            let start = Instant::now();
            board.solve();
            println!("Solved board in {:?}", start.elapsed());
            println!("{}", board);
        } else {
            println!("{} is not a valid id", buffer);
        }
        buffer.clear();
    }
}

