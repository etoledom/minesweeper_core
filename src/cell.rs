use crate::graphics::Point;

#[derive(Debug, Copy, Clone, Default)]
pub struct Cell {
    pub number: i8,
    pub cleared: bool,
    pub flagged: bool,
    pub coordinates: Point,
}

impl Cell {
    pub fn is_mine(&self) -> bool {
        self.number.is_negative()
    }

    pub fn new_mine(coordinates: Point) -> Cell {
        Cell {
            number: -1,
            coordinates,
            ..Default::default()
        }
    }
}
