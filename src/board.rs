use std::ops::Index;
use std::fmt;
use crate::{Point,PointSet,dictionary};
use colored::{Colorize, ColoredString};

#[derive(Clone)]
pub struct Board {
    letters: [[u8; 7]; 12],
    word_ids: [[i8; 7]; 12],
}

impl Board {
    fn new() -> Board {
        Board {
            letters: [[0; 7]; 12],
            word_ids: [[-1; 7]; 12],
        }
    }

    fn visited(&self, x: usize, y: usize) -> bool {
        self.word_ids[y][x] != -1
    }

    /// Test if the board is completely filled in
    pub fn is_done(&self) -> bool {
        self.word_ids.iter().all(|row| row.iter().all(|&id| id != -1))
    }

    /// Return number of defined words
    pub fn n_words(&self) -> usize {
        let mut max: i8 = -1;
        for y in 0..12 {
            for x in 0..7 {
                if self.word_ids[y][x] > max {
                    max = self.word_ids[y][x]
                }
            }
        }
        (max + 1) as usize
    }

    /// Add the given point set to the current board
    fn insert_word(&mut self, points: &PointSet) {
        let word_id = self.n_words() as i8;
        points.clone().into_iter()
            .for_each(|p| self.word_ids[p.y as usize][p.x as usize] = word_id);
    }

    /// Remove the given point set
    /// It is up to the user to only call this on the most recent set of points
    fn remove_word(&mut self, points: &PointSet) {
        points.clone().into_iter()
            .for_each(|p| self.word_ids[p.y as usize][p.x as usize] = -1);
    }

    /// Return the uppermost, leftmost, currently unmarked point
    fn get_root(&self) -> Option<Point> {
        for y in 0..12 {
            for x in 0..7 {
                if self.word_ids[y][x] == -1 {
                    return Some(Point::new(x as u8, y as u8));
                }
            }
        }
        return None;
    }

    /// Return a list of all valid solutions to the board
    pub fn enumerate_solutions(&mut self) -> Vec<Board> {
        let mut solutions = Vec::new();

        self.enumerate_solutions_inner(&mut solutions);

        return solutions;
    }

    fn enumerate_solutions_inner(&mut self, solutions: &mut Vec<Board>) {
        for word in self.next_words().iter() {
            self.insert_word(word);
            if self.is_done() {
                solutions.push(self.clone());
            } else {
                self.enumerate_solutions_inner(solutions)
            }
            self.remove_word(word);
        }
    }

    /// Solve the board
    pub fn solve(&mut self) -> bool {
        println!("Solving:");
        println!("{}", self);

        for word in self.next_words().iter() {
            self.insert_word(word);
            if self.is_done() {
                return true;
            } else if self.solve() {
                return true;
            }
            self.remove_word(word);
        }
        return false;
    }


    /// For a given state, return a list of valid next words
    /// TODO, return a &'static [u8] from the tree
    pub fn next_words(&self) -> Vec<PointSet> {
        let root_point = self.get_root().unwrap();
        let points = PointSet::new(root_point);
        let mut words = Vec::new();
        let dict_node = dictionary::first_node(self[root_point]).unwrap();
        self.list_words(points, dict_node, &mut words);
        return words;
    }

    /// Recursively enumerate all possible words
    pub fn list_words(
        &self,
        points: PointSet,
        dict_node: &'static dictionary::Node,
        list: &mut Vec<PointSet>
    ) {
        if !points.connectable() {
            // If the given points aren't connectable, return early
            return;
        }
        // First check if we're currently a word
        if dict_node.is_word && points.contiguous() {
            list.push(points.clone());
        }

        if points.length() < 8 {
            // Try to add 1 character to the word
            let last_point = points.last_point();
            for x in last_point.x+1..7 {
                if self.visited(x as usize, last_point.y as usize) {
                    continue;
                }
                // Consider point (x, last_point.y)
                let letter = self.letters[last_point.y as usize][x as usize];
                if let Some(next_node) = dict_node.get_next(letter) {
                    let mut next_points = points.clone();
                    next_points.push(Point::new(x, last_point.y));
                    self.list_words(next_points, next_node, list);
                }
            }
            if last_point.y < 11 {
                let y = last_point.y + 1;
                for x in 0..=last_point.x {
                    if self.visited(x as usize, y as usize) {
                        continue;
                    }
                    let letter = self.letters[y as usize][x as usize];
                    if let Some(next_node) = dict_node.get_next(letter) {
                        let mut next_points = points.clone();
                        next_points.push(Point::new(x, y));
                        self.list_words(next_points, next_node, list);
                    }
                }
            }
        }
    }

    /// Test if a horizontal connection exists between (x, y) and (x+1, y)
    fn is_h_connection(&self, x: usize, y: usize) -> bool {
        debug_assert!(x <= 5);
        debug_assert!(y <= 11);
        self.word_ids[y][x] != -1 && (self.word_ids[y][x] == self.word_ids[y][x+1])
    }

    /// Test if a vertical connection exists between (x, y) and (x, y+1)
    fn is_v_connection(&self, x: usize, y: usize) -> bool {
        debug_assert!(x <= 6);
        debug_assert!(y <= 10);
        self.word_ids[y][x] != -1 && (self.word_ids[y][x] == self.word_ids[y+1][x])
    }

    /// Return a colored format of the given letter index
    fn disp_char(&self, x: usize, y: usize) -> ColoredString {
        let buf = [self.letters[y][x]; 1];
        let s = std::str::from_utf8(&buf).unwrap();
        if self.word_ids[y][x] == -1 {
            s.normal()
        } else {
            s.reversed()
        }
    }

    /// Return a colored format of the given horizontal connector
    fn disp_h_conn(&self, x: usize, y: usize) -> ColoredString {
        if self.is_h_connection(x, y) {
            " ".reversed()
        } else {
            " ".normal()
        }
    }

    /// Return a colored format of the given horizontal connector
    fn disp_v_conn(&self, x: usize, y: usize) -> ColoredString {
        if self.is_v_connection(x, y) {
            " ".reversed()
        } else {
            " ".normal()
        }
    }

    /// Print the given row to the formatter
    fn fmt_row(&self, f: &mut fmt::Formatter<'_>, index: usize) -> fmt::Result {
        for x in 0..6 {
            write!(f, "{}{}", self.disp_char(x, index), self.disp_h_conn(x, index))?;
        }
        writeln!(f, "{}", self.disp_char(6, index))
    }

    /// Print the row of connections
    fn fmt_connect_row(&self, f: &mut fmt::Formatter<'_>, index: usize) -> fmt::Result {
        for x in 0..6 {
            write!(f, "{} ", self.disp_v_conn(x, index))?;
        }
        writeln!(f, "{}", self.disp_v_conn(6, index))
    }
}

impl Index<Point> for Board {
    type Output = u8;

    fn index(&self, index: Point) -> &Self::Output {
        &self.letters[index.y as usize][index.x as usize]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..11 {
            self.fmt_row(f, y)?;
            self.fmt_connect_row(f, y)?;
        }
        self.fmt_row(f, 11)
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(u8),
    MissingChar{line: usize, character: usize},
}

impl TryFrom<&str> for Board {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut res = Board::new();
        let mut bytes = value.as_bytes().iter();
        for y in 0..12 {
            for x in 0..7 {
                match bytes.next() {
                    Some(&c) if c >= b'a' && c <= b'z' => res.letters[y][x] = c,
                    Some(&c) => return Err(ParseError::UnexpectedChar(c)),
                    None => return Err(ParseError::MissingChar{ line: y, character: x }),
                };
            }
            match bytes.next() {
                Some(b'\n') => {},
                Some(&c) => return Err(ParseError::UnexpectedChar(c)),
                None => return Err(ParseError::MissingChar{ line: y, character: 7 }),
            }
        }
        Ok(res)
    }
}
