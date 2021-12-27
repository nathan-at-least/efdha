use crate::hex::coordinates::Relative;

mod iter;

#[cfg(test)]
mod tests;

pub use self::iter::DirIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    West,
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn iter() -> DirIter {
        DirIter::new()
    }

    pub fn clockwise(self) -> Direction {
        use Direction::*;

        match self {
            West => NorthWest,
            NorthWest => NorthEast,
            NorthEast => East,
            East => SouthEast,
            SouthEast => SouthWest,
            SouthWest => West,
        }
    }

    pub fn counterclockwise(self) -> Direction {
        use Direction::*;

        match self {
            West => SouthWest,
            NorthWest => West,
            NorthEast => NorthWest,
            East => NorthEast,
            SouthEast => East,
            SouthWest => SouthEast,
        }
    }
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

impl std::ops::Mul<i64> for Direction {
    type Output = Relative;

    fn mul(self, d: i64) -> Relative {
        let mut r = Relative::from(self);
        r *= d;
        r
    }
}
