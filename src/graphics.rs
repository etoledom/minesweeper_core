use std::mem;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Vec2<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Vec2<T>
where
    T: Copy,
    T: Clone,
{
    pub fn new(data: Vec<Vec<T>>) -> Vec2<T> {
        Vec2 { data }
    }
}

impl<T> Vec2<T> {
    pub fn for_each_element(&self, mut f: impl FnMut(Point, &T, &mut bool)) {
        let mut stop = false;
        for x in 0..self.data.len() {
            for y in 0..self.data[x].len() {
                f(Point { x, y }, &self.data[x][y], &mut stop);
                if stop {
                    break;
                }
            }
            if stop {
                break;
            }
        }
    }

    pub fn for_each_element_mut(&mut self, mut f: impl FnMut(Point, &mut T, &mut bool)) {
        let mut stop = false;
        for x in 0..self.data.len() {
            for y in 0..self.data[x].len() {
                f(Point { x, y }, &mut self.data[x][y], &mut stop);
                if stop {
                    break;
                }
            }
            if stop {
                break;
            }
        }
    }

    pub fn get_width(&self) -> usize {
        self.data.len()
    }

    pub fn get_height(&self) -> usize {
        let row = self.data.get(0);
        match row {
            Some(cell) => cell.len(),
            None => 0,
        }
    }

    pub fn get_element(&self, coordinates: Point) -> Option<&T> {
        if let Some(cell) = self.data.get(coordinates.x)?.get(coordinates.y) {
            return Some(cell);
        }
        None
    }

    pub fn replace_at(&mut self, element: T, coordinates: Point) {
        // TODO: make it safe
        _ = mem::replace(&mut self.data[coordinates.x][coordinates.y], element);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn random_between(
        range_x: std::ops::Range<usize>,
        range_y: std::ops::Range<usize>,
    ) -> Point {
        use rand::prelude::thread_rng;
        let mut rng = thread_rng();

        Point {
            x: rng.gen_range(range_x),
            y: rng.gen_range(range_y),
        }
    }

    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
