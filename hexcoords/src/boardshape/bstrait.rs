use crate::hex::coordinates::Absolute;

pub trait BoardShape {
    fn in_bounds(&self, coords: Absolute) -> bool;
}
