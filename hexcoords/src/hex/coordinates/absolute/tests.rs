use crate::hex;

#[test]
fn travel_circuit() {
    let start = hex::coordinates::Absolute(42, 37);

    // Test for multiple side lengths:
    for sidelen in 0..12 {
        // Test turns of 1, 2, or 3 clockwise rotations per side:
        for turn in 1..4 {
            // Test starting from each direction:
            for startdir in hex::Direction::iter() {
                let mut t = start;
                let mut dir = startdir;

                // There are 6 sides for turn 1, 3 for turn 2, and 2 for turn 3:
                for _side in 0..6 / turn {
                    // Step in dir for sidelen steps:
                    for _step in 0..sidelen {
                        t += dir;
                    }

                    // Turn clockwise turn times:
                    for _ in 0..turn {
                        dir = dir.clockwise();
                    }
                }

                // We should have returned to the starting location:
                assert_eq!(start, t);
            }
        }
    }
}
