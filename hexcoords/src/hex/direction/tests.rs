use crate::hex::Direction;

#[test]
fn clockwise_counterclockwise_inverse() {
    for d in Direction::iter() {
        let e = d.clockwise();
        let f = e.counterclockwise();
        assert_eq!(d, f);

        let g = d.counterclockwise();
        let h = g.clockwise();
        assert_eq!(d, h);
    }
}
