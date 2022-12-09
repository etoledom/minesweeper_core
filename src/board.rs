use crate::graphics::*;
use crate::Cell;

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec2<Cell>,
}

impl Board {
    // INIT:

    pub fn new_empty(size: Size) -> Self {
        let mut cells = Vec::new();
        for x in 0..size.width {
            let mut row = Vec::new();
            for y in 0..size.height {
                let cell = Cell {
                    coordinates: Point { x, y },
                    ..Default::default()
                };
                row.push(cell);
            }
            cells.push(row);
        }
        Board::new_with_cells(Vec2 { data: cells })
    }

    pub fn new(mines: i32, size: Size) -> Self {
        let mut board = Board::new_empty(size);
        board.add_mines(mines).add_cell_numbers();
        board
    }

    pub fn new_with_cells(cells: Vec2<Cell>) -> Board {
        Board { cells }
    }

    // Populate cells:

    fn add_mines(&mut self, mines: i32) -> &mut Self {
        if mines == 0 {
            return self;
        }
        let coordinates = Point::random_between(0..self.get_width(), 0..self.get_height());

        let Some(cell) = self.cell_at(coordinates) else {
            return self.add_mines(mines);
        };

        if cell.is_mine() {
            return self.add_mines(mines);
        }

        let mine = Cell::new_mine(coordinates);
        self.replace_cell(mine, mine.coordinates);

        self.add_mines(mines - 1)
    }

    pub fn add_cell_numbers(&mut self) -> &mut Self {
        self.clone().for_each_cell(|point, cell, _| {
            if !cell.is_mine() {
                self.count_mines_around_cell_at(point);
            }
        });
        self
    }

    fn count_mines_around_cell_at(&mut self, coordinates: Point) {
        let count_mines = || {
            let mut count: i8 = 0;
            self.get_cells_around(coordinates).iter().for_each(|cell| {
                if cell.is_mine() {
                    count += 1;
                }
            });
            count
        };

        self.replace_cell(
            Cell {
                number: count_mines(),
                coordinates,
                ..Default::default()
            },
            coordinates,
        );
    }

    // PUBLIC:

    pub fn cell_at(&self, coordinates: Point) -> Option<&Cell> {
        self.cells.get_element(coordinates)
    }

    pub fn get_width(&self) -> usize {
        self.cells.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.cells.get_height()
    }

    pub fn get_size(&self) -> Size {
        Size {
            width: self.get_width(),
            height: self.get_height(),
        }
    }

    pub fn replace_cell(&mut self, new_cell: Cell, coordinates: Point) {
        self.cells.replace_at(new_cell, coordinates);
    }

    pub fn for_each_cell(&self, f: impl FnMut(Point, &Cell, &mut bool)) {
        self.cells.for_each_element(f);
    }

    pub fn for_each_cell_mut(&mut self, f: impl FnMut(Point, &mut Cell, &mut bool)) {
        self.cells.for_each_element_mut(f);
    }

    pub fn get_cells_around(&self, coordinates: Point) -> Vec<&Cell> {
        let (x, y) = (coordinates.x, coordinates.y);
        let mut cells: Vec<&Cell> = vec![];

        for dx in [-1i32, 0, 1] {
            for dy in [-1i32, 0, 1] {
                if x as i32 + dx < 0 || y as i32 + dy < 0 || (dx == 0 && dy == 0) {
                    continue;
                }
                let coordinates = Point {
                    x: (x as i32 + dx) as usize,
                    y: (y as i32 + dy) as usize,
                };
                if let Some(cell) = self.cell_at(coordinates) {
                    cells.push(cell);
                }
            }
        }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_left_cell() {
        let board = get_board_with_number_top_left();
        let Some(cell) = board.cell_at(Point::zero()) else {
            panic!()
        };
        assert!(cell.number > 0);
    }

    #[test]
    fn test_get_cells_around_top_left_cell() {
        let board = Board::new(
            1,
            Size {
                width: 3,
                height: 3,
            },
        );
        let cells = board.get_cells_around(Point::zero());
        assert_eq!(cells.len(), 3);
    }

    #[test]
    fn test_get_cells_around_central_cell() {
        let board = Board::new(
            1,
            Size {
                width: 3,
                height: 3,
            },
        );
        let cells = board.get_cells_around(Point { x: 1, y: 1 });
        assert_eq!(cells.len(), 8);
    }

    #[test]
    fn test_get_cells_around_bottom_right_cell() {
        let board = Board::new(
            1,
            Size {
                width: 3,
                height: 3,
            },
        );
        let cells = board.get_cells_around(Point { x: 2, y: 2 });
        assert_eq!(cells.len(), 3);
    }

    /// Get a board where the top-left cell must be a number.
    fn get_board_with_number_top_left() -> Board {
        let board = Board::new(
            1,
            Size {
                width: 2,
                height: 2,
            },
        );
        if let Some(cell) = board.cell_at(Point::zero()) {
            if cell.is_mine() {
                return get_board_with_number_top_left();
            }
        }
        board
    }
}
