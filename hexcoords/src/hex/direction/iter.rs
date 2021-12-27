use crate::hex::Direction;

#[derive(Debug)]
pub struct DirIter(Option<Direction>);

impl DirIter {
    pub fn new() -> Self {
        DirIter(Some(Direction::West))
    }
}

impl Iterator for DirIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        use Direction::*;

        let optitem = self.0;
        self.0 = match optitem {
            None => None,
            Some(SouthWest) => None,
            Some(d) => Some(d.clockwise()),
        };

        optitem
    }
}
