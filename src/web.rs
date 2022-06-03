use isahc::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Puzzle {
    pub width: usize,
    pub height: usize,
    pub minSize: usize,
    pub maxSize: usize,
    pub regions: Vec<Vec<[usize; 2]>>,
    pub words: Vec<String>,
}

impl Puzzle {
    /// Load a puzzle from it's id number
    pub fn from_id(id: usize) -> Result<Puzzle, isahc::Error> {
        let url = format!("https://www.andrewt.net/puzzles/cell-tower/puzzles/{}.json", id);
        Ok(isahc::get(url)?.json().unwrap())
    }
}

