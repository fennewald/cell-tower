
struct Puzzle {
    width: usize,
    height: usize,
    minSize: usize,
    maxSize: usize,
    regions: Vec<Vec<[usize; 2]>>,
    words: Vec<String>,
}

impl Puzzle {
    /// Load a puzzle from it's id number
    fn from_id(id: usize) -> Result<Puzzle> {
        let url = format!("https://www.andrewt.net/puzzles/cell-tower/puzzles/{}.json", id);
    }
}

