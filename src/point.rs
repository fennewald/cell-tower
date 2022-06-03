use smallvec::SmallVec;
use crate::Bitset;
use std::fmt;

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    /// Create a new point
    pub fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }

    /// Return a new point, 1 to the left
    pub fn left(&self) -> Option<Point> {
        if self.x == 0 {
            None
        } else {
            Some(Point{ x: self.x-1, y: self.y })
        }
    }

    /// Return a new point, 1 to the right
    pub fn right(&self) -> Option<Point> {
        if self.x == 6 {
            None
        } else {
            Some(Point{ x: self.x+1, y: self.y })
        }
    }

    /// Return a new point, 1 above
    pub fn above(&self) -> Option<Point> {
        if self.y == 0 {
            None
        } else {
            Some(Point{ x: self.x, y: self.y-1 })
        }
    }

    /// Return a new point, 1 below
    pub fn below(&self) -> Option<Point> {
        if self.y == 11 {
            None
        } else {
            Some(Point{ x: self.x, y: self.y+1 })
        }
    }

    /// Test if self is on the edge defined by edge_root
    pub fn on_edge(&self, edge_root: &Point) -> bool {
        (self.y == edge_root.y && self.x <= edge_root.x) || (self.y+1 == edge_root.y && self.x > edge_root.x)
    }

    /// Test if self is beyond a given point
    pub fn is_beyond(&self, other: &Point) -> bool {
        self.y > other.y || (self.y == other.y && self.x > other.x)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct PointSet (SmallVec<[Point; 8]>);

impl PointSet {
    /// Construct a new PointSet, with the single, given, point
    pub fn new(root: Point) -> PointSet {
        let mut points = SmallVec::with_capacity(8);
        points.push(root);
        PointSet(points)
    }

    pub fn push(&mut self, point: Point) {
        self.0.push(point)
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    /// Return a bitset, where each valid point is marked with a true
    fn make_bitset(&self) -> Bitset {
        Bitset::first_n(self.length())
    }

    /// Determine the index of the provided point
    fn index_of(&self, point: Point) -> Option<usize> {
        self.0.iter().position(|&p| p == point)
    }

    /// Mark the given index as visited
    fn mark_index(&self, index: usize, visited: &mut Bitset) {
        if !visited.get(index) {
            visited.set(index, true);
            let point = self.0[index as usize];

            if let Some(neighbor) = point.above() {
                if let Some(n_index) = self.index_of(neighbor) {
                    self.mark_index(n_index, visited);
                }
            }
            if let Some(neighbor) = point.below() {
                if let Some(n_index) = self.index_of(neighbor) {
                    self.mark_index(n_index, visited);
                }
            }
            if let Some(neighbor) = point.left() {
                if let Some(n_index) = self.index_of(neighbor) {
                    self.mark_index(n_index, visited);
                }
            }
            if let Some(neighbor) = point.right() {
                if let Some(n_index) = self.index_of(neighbor) {
                    self.mark_index(n_index, visited);
                }
            }
        }
    }

    /// Mark the first point within the bitset as vistied
    fn mark_first(&self, vistied: &mut Bitset) {
        self.mark_index(0, vistied)
    }

    /// Return the last point inside of self
    pub fn last_point(&self) -> Point {
        debug_assert!(self.length() > 0);
        self.0[self.length() - 1]
    }

    /// Mark all points on the bottom edge
    /// Defined as:
    /// ```
    /// . . s x x x x x
    /// x x x
    /// ```
    fn mark_edge(&self, visited: &mut Bitset) {
        let last_point = self.last_point();
        // For each point, check if it's on the edge
        for i in 0..self.length() {
            if !visited.get(i) && self.0[i].on_edge(&last_point) {
                self.mark_index(i, visited);
            }
        }
    }

    /// Test if the current set of points are all connected
    pub fn contiguous(&self) -> bool {
        let mut visited = self.make_bitset();
        self.mark_first(&mut visited);
        visited.is_full()
    }

    /// Test if the points could be weak connected
    /// This assumes that the user may place infinite points to the right or
    /// below the final selected character.
    ///
    /// When this returns false, there is no way to connect the points
    /// When this returns true, there may be a way to conenct the points
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
    pub fn connectable(&self) -> bool {
        let mut visited = self.make_bitset();

        self.mark_edge(&mut visited);

        if visited.get(0) {
            visited.is_full()
        } else if visited != self.make_bitset() {
            false
        } else {
            self.mark_first(&mut visited);
            visited.is_full()
        }
    }
}

impl IntoIterator for PointSet {
    type Item = Point;
    type IntoIter = smallvec::IntoIter<[Point; 8]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<&[Point]> for PointSet {
    fn from(slice: &[Point]) -> PointSet {
        PointSet(SmallVec::from_slice(slice))
    }
}

impl fmt::Display for PointSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0.as_slice())
    }
}

impl fmt::Debug for PointSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Set{{{:?}}}", self.0.as_slice())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Convenience function
    fn p(x: u8, y: u8) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn one_point() {
        let set: PointSet = [p(1, 1)].as_slice().into();
        assert!(set.connectable());
        assert!(set.contiguous());
    }

    #[test]
    fn pair_point() {
        let set: PointSet = [p(1, 1), p(1,2)].as_slice().into();
        assert!(set.connectable());
        assert!(set.contiguous());
    }

    #[test]
    fn case_0() {
        let set: PointSet = [p(1, 1), p(3, 1), p(2,2)].as_slice().into();
        assert!(!set.connectable());
        assert!(!set.contiguous());
    }
}
