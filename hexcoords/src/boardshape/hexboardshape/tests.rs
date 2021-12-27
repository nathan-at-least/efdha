use crate::{hex, BoardShape, HexBoardShape};

#[test]
fn in_bounds_0() {
    test_bounds(0, &[((0, 0), true), ((0, 1), false), ((1, 1), false)]);
}

#[test]
fn in_bounds_1() {
    test_bounds(
        1,
        &[
            ((-1, -1), false),
            ((-1, 0), true),
            ((-1, 1), true),
            ((0, -1), true),
            ((0, 0), true),
            ((0, 1), true),
            ((1, -1), true),
            ((1, 0), true),
            ((1, 1), false),
            ((-2, 0), false),
            ((0, 2), false),
        ],
    );
}

#[test]
fn in_bounds_2_spot_check() {
    test_bounds(2, &[((-1, -1), true), ((-2, 0), true), ((2, 2), false)]);
}

fn test_bounds(radius: u64, testvec: &[((i64, i64), bool)]) {
    let hbs = HexBoardShape::with_radius(radius);
    for &((w, v), inb) in testvec {
        assert_eq!(inb, hbs.in_bounds(hex::coordinates::Absolute(w, v)));
    }
}
