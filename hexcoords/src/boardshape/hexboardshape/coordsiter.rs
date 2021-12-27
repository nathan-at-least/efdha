use crate::hex::{coordinates::Absolute, DirIter, Direction};

#[derive(Debug)]
pub struct CoordsIter {
    radius: u64,
    stepsleft: u64,
    diriter: DirIter,
    dir: Direction,
    optspot: Option<Absolute>,
}

impl CoordsIter {
    pub fn new(radius: u64) -> Self {
        let stepsleft = radius + 1;
        let mut diriter = Direction::iter();
        let dir = diriter.next().unwrap();
        let optspot = new_edge(radius, dir);

        CoordsIter {
            radius,
            stepsleft,
            diriter,
            dir,
            optspot,
        }
    }

    fn next_from_spot(&mut self, spot: Absolute) -> Option<Absolute> {
        if self.radius == 0 && self.stepsleft == 0 {
            None
        } else {
            if self.stepsleft > 0 {
                self.stepsleft -= 1;
                self.optspot = Some(spot + self.dir);
            } else {
                match self.diriter.next() {
                    None => {
                        *self = CoordsIter::new(self.radius - 1);
                    }
                    Some(d) => {
                        self.stepsleft = self.radius + 1;
                        self.dir = d;
                        self.optspot = new_edge(self.radius, d);
                    }
                }
            }
            Some(spot)
        }
    }
}

impl Iterator for CoordsIter {
    type Item = Absolute;

    fn next(&mut self) -> Option<Self::Item> {
        self.optspot.and_then(|s| self.next_from_spot(s))
    }
}

fn new_edge(radius: u64, dir: Direction) -> Option<Absolute> {
    let ir = i64::try_from(radius).unwrap();
    let tmpdir = dir.counterclockwise().counterclockwise();
    Some(Absolute(0, 0) + (tmpdir * ir))
}
