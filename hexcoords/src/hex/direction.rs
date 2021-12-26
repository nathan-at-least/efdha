use crate::hex::coordinates::Relative;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    West,
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
}

impl TryFrom<Relative> for Direction {
    type Error = ();

    fn try_from(rc: Relative) -> Result<Direction, ()> {
        use Direction::*;

        let Relative(v, w) = rc;
        match (v, w) {
            (-1, 0) => Ok(West),
            (-1, 1) => Ok(NorthWest),
            (0, 1) => Ok(NorthEast),
            (1, 0) => Ok(East),
            (1, -1) => Ok(SouthEast),
            (0, -1) => Ok(SouthWest),
            _ => Err(()),
        }
    }
}
