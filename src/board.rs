use crate::Cell;

#[derive(Copy,Clone,PartialEq,Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    /// Create a new point
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    /// Return a new point, 1 to the left
    fn left(&self) -> Option<Point> {
        if self.x == 0 {
            None
        } else {
            Some(Point{ x: self.x-1, y: self.y })
        }
    }

    /// Return a new point, 1 to the right
    fn right(&self) -> Option<Point> {
        if self.x == 6 {
            None
        } else {
            Some(Point{ x: self.x+1, y: self.y })
        }
    }

    /// Return a new point, 1 above
    fn above(&self) -> Option<Point> {
        if self.y == 0 {
            None
        } else {
            Some(Point{ x: self.x, y: self.y-1 })
        }
    }

    /// Return a new point, 1 below
    fn below(&self) -> Option<Point> {
        if self.y == 11 {
            None
        } else {
            Some(Point{ x: self.x, y: self.y+1 })
        }
    }
}

#[derive(Clone)]
struct PointSet {
    points: [Option<Point>; 8],
    length: u8,
}

impl PointSet {
    fn new(root: Point) -> PointSet {
        let mut res = PointSet {
            points: [None; 8],
            length: 1,
        };
        res.points[0] = Some(root);
        return res;
    }
    /// Test if the root point has been reached
    fn first_reached(&self) -> bool {
        self.points[0] == None
    }

    /// Mark the first point of the set connected
    fn mark_first_connected(&mut self) {
        debug_assert!(self.points[0] != None);
        self.connect_index(0)
    }

    /// Get the index of the provided point
    fn index_of(&self, point: Point) -> Option<usize> {
        self.points.iter().position(|&p| p == Some(point))
    }

    /// Mark the provided point as connected
    fn connect(&mut self, point: Point) {
        if let Some(index) = self.index_of(point) {
            self.connect_index(index);
        }
    }

    /// Mark the point at the index connected
    /// Recursively mark all connected points invalid
    fn connect_index(&mut self, index: usize) {
        if let Some(coords) = self.points[index] {
            self.length -= 1;
            self.points[index] = None;
            if let Some(p) = coords.left() {
                self.connect(p);
            }
            if let Some(p) = coords.right() {
                self.connect(p);
            }
            if let Some(p) = coords.above() {
                self.connect(p);
            }
            if let Some(p) = coords.above() {
                self.connect(p);
            }
        }
    }

    /// Is the current list empty
    fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Mark all points beyond the given point as connected
    fn connect_beyond(&mut self, root: Point) {
        debug_assert!(self.points[0] != None);
        for x in root.x+1..7 {
            self.connect(Point::new(x, root.y));
        }
        for x in 0..=root.x {
            self.connect(Point::new(x, root.y+1));
        }
    }

    /// Insert the given point into the internal list
    fn insert(&mut self, point: Point) {
        debug_assert!(self.length <= 7);
        self.points[self.length as usize] = Some(point);
        self.length += 1;
    }
}

impl From<&[Point]> for PointSet {
    fn from(slice: &[Point]) -> PointSet {
        let mut src = slice.iter();
        let mut res = PointSet { points: [None; 8], length: 0 };
        for i in 0..8 {
            if let Some(p) = src.next() {
                res.points[i] = Some(*p);
            }
        }
        return res;
    }
}

pub struct Board {
    letters: [[u8; 7]; 12],
    word_id: [[i8; 7]; 12],
}

impl Board {
    /// Is the current set of selected cells connectable
    ///
    /// Key: . -> unselected, s -> selected, x -> eligible for connections
    /// ```
    /// . . s . . . s .
    /// s s s s x x x x    Connectable
    /// x x x x
    ///
    /// . s . . . s . .
    /// . s s . . . . .    Impossible
    /// . . s x x x x x
    /// x x x
    /// ```
    ///
    pub fn connectable(&self, selected: &PointSet) -> bool {
        let mut points: PointSet = selected.clone();
        let last_point = selected[selected.len() - 1];
        points.connect_beyond(last_point);

        // TODO perf the statement below
        if points.first_reached() {
            return points.length == 0;
        } else if points.length != selected.len() as u8 {
            // There were points marked using the edge that couldn't reach the
            // root. It is impossible to win
            return false;
        } else {
            points.mark_first_connected();
            return points.length == 0;
        }
    }

    pub fn solve(&mut self) -> Option<()> {
        // Select the uppermost, leftmost point
        let mut selected = PointSet::new(self.first_unselected()?);

        for i in 1..8 {
            // Select a point from right on
        }
        Some(())
    }

    /// Return the leftmost, uppermost point
    fn first_unselected(&self) -> Option<Point> {
        for y in 0..12 {
            for x in 0..8 {
                if self.word_id[y][x] == -1 {
                    return Some(Point::new(x, y));
                }
            }
        }
        return None;
    }

    /// Return the set of unselected points beyond root
    /// Filters for points that are
    fn get_set_beyond(&self, root: Point, prefix: &[u8]) -> Vec<Point> {
        let mut beyond = Vec::with_capacity(8);
        for x in root.x+1..7 {
            if self.word_id[root.y][x] != -1 {
                continue;
            }
            if
        }
        for x in 0..=root.x {
        }
        beyond
    }
}
