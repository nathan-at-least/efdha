use crate::hex::coordinates::Absolute;
use crate::BoardShape;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HexBoardShape(u64);

impl HexBoardShape {
    pub fn with_radius(radius: u64) -> HexBoardShape {
        HexBoardShape(radius)
    }
}

impl BoardShape for HexBoardShape {
    fn in_bounds(&self, coords: Absolute) -> bool {
        todo!("{:?}.in_bounds({:?})", self, coords);
    }
}
