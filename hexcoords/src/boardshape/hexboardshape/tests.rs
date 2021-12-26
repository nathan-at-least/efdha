use crate::{hex, BoardShape, HexBoardShape};

#[test]
fn in_bounds() {
    let hbs = HexBoardShape::with_radius(0);
    assert_eq!(false, hbs.in_bounds(hex::coordinates::Absolute(1, 0)));
}
