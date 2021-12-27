use crate::hex::coordinates::Absolute;
use crate::BoardShape;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HexBoardShape(i64);

impl HexBoardShape {
    pub fn with_radius(radius: i64) -> HexBoardShape {
        assert!(radius >= 0);
        HexBoardShape(radius)
    }
}

impl BoardShape for HexBoardShape {
    fn in_bounds(&self, coords: Absolute) -> bool {
        let HexBoardShape(radius) = *self;
        let Absolute(w, v) = coords;

        w.abs() <= radius && v.abs() <= radius && (w + v).abs() <= radius
    }
}
