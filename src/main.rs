extern crate smallvec;
extern crate colored;

mod bitset;
mod board;
mod dictionary;
mod point;
//mod solver;

pub use bitset::Bitset;
pub use board::Board;
pub use point::{Point,PointSet};

fn main() {
    let src = "rsupese\neriporn\nliesdin\nougwagh\nhtbrnap\necominp\naningge\nypcodhn\nlacseer\nemlowed\nullyedu\ntiplece\n";
    let mut b: Board = src.try_into().unwrap();
    let solutions = b.enumerate_solutions();
    println!("Found {} solutions", solutions.len());
    for solution in solutions {
        println!("{}", solution);
    }
}

// rsupese
// eriporn
// liesdin
// ougwagh
// htbrnap
// ecominp
// aningge
// ypcodhn
// lacseer
// emlowed
// ullyedu
// tiplece

