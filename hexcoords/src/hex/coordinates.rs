use crate::hex::Direction;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coordinates(pub i64, pub i64);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RelCoords(pub i64, pub i64);

impl From<Direction> for RelCoords {
    fn from(d: Direction) -> RelCoords {
        use Direction::*;

        let (v, w) = match d {
            West => (-1, 0),
            NorthWest => (-1, 1),
            NorthEast => (0, 1),
            East => (1, 0),
            SouthEast => (1, -1),
            SouthWest => (0, -1),
        };

        RelCoords(v, w)
    }
}
