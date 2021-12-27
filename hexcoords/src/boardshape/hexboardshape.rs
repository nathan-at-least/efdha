mod coordsiter;

#[cfg(test)]
mod tests;

use crate::hex::coordinates::Absolute;
use crate::BoardShape;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HexBoardShape(u64);

impl HexBoardShape {
    pub fn with_radius(radius: u64) -> HexBoardShape {
        assert!(
            i64::try_from(radius).is_ok(),
            "Invalid radius: {:?}",
            radius
        );
        HexBoardShape(radius)
    }
}

impl BoardShape for HexBoardShape {
    type Coords = self::coordsiter::CoordsIter;

    fn in_bounds(&self, coords: Absolute) -> bool {
        let HexBoardShape(uradius) = *self;
        let Absolute(w, v) = coords;

        let radius = i64::try_from(uradius).unwrap();
        w.abs() <= radius && v.abs() <= radius && (w + v).abs() <= radius
    }

    fn coords(&self) -> Self::Coords {
        self::coordsiter::CoordsIter::new(self.0)
    }
}
