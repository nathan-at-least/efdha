use crate::hex::coordinates::Absolute;

pub trait BoardShape {
    type Coords: Iterator<Item = Absolute>;

    fn in_bounds(&self, coords: Absolute) -> bool;
    fn coords(&self) -> Self::Coords;
}
