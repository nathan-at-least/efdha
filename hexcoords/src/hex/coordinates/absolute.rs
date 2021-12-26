use crate::hex::coordinates::Relative;
use crate::hex::Direction;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Absolute(pub i64, pub i64);

impl std::ops::AddAssign<Direction> for Absolute {
    fn add_assign(&mut self, rhs: Direction) {
        *self += Relative::from(rhs)
    }
}

impl std::ops::Add<Direction> for Absolute {
    type Output = Absolute;

    fn add(self, rhs: Direction) -> Absolute {
        self + Relative::from(rhs)
    }
}

impl std::ops::AddAssign<Relative> for Absolute {
    fn add_assign(&mut self, rhs: Relative) {
        let Relative(dv, dw) = rhs;
        self.0 += dv;
        self.1 += dw;
    }
}

impl std::ops::Add<Relative> for Absolute {
    type Output = Absolute;

    fn add(self, rhs: Relative) -> Absolute {
        let mut sum = self;
        sum += rhs;
        sum
    }
}

impl std::ops::Sub<Absolute> for Absolute {
    type Output = Relative;

    fn sub(self, rhs: Absolute) -> Relative {
        let Absolute(v1, w1) = self;
        let Absolute(v2, w2) = rhs;
        Relative(v1 - v2, w1 - w2)
    }
}
