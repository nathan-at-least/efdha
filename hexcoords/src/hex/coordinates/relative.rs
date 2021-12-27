use crate::hex::Direction;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Relative(pub i64, pub i64);

impl From<Direction> for Relative {
    fn from(d: Direction) -> Relative {
        use Direction::*;

        let (v, w) = match d {
            West => (-1, 0),
            NorthWest => (-1, 1),
            NorthEast => (0, 1),
            East => (1, 0),
            SouthEast => (1, -1),
            SouthWest => (0, -1),
        };

        Relative(v, w)
    }
}

impl std::ops::Mul<i64> for Relative {
    type Output = Relative;

    fn mul(self, d: i64) -> Relative {
        let mut r = self;
        r *= d;
        r
    }
}

impl std::ops::MulAssign<i64> for Relative {
    fn mul_assign(&mut self, d: i64) {
        self.0 *= d;
        self.1 *= d;
    }
}
