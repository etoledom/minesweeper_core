use crate::board::Board;
use crate::cell::Cell;
use crate::graphics::*;

struct GameConfiguration {
    mines_count: i32,
    size: Size,
}

impl GameConfiguration {
    fn easy() -> GameConfiguration {
        GameConfiguration {
            mines_count: 11,
            size: Size { height: 10, width: 10 },
        }
    }

    fn medium() -> GameConfiguration {
        GameConfiguration {
            mines_count: 41,
            size: Size { height: 16, width: 16 },
        }
    }

    fn hard() -> GameConfiguration {
        GameConfiguration {
            mines_count: 99,
            size: Size { height: 16, width: 30 },
        }
    }

    fn configuration_for(difficulty: Difficulty) -> GameConfiguration {
        match difficulty {
            Difficulty::Easy => GameConfiguration::easy(),
            Difficulty::Medium => GameConfiguration::medium(),
            Difficulty::Hard => GameConfiguration::hard(),
        }
    }
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub total_mines: i32,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Game {
        let config = GameConfiguration::configuration_for(difficulty);
        Game {
            board: Board::new(config.mines_count, config.size),
            total_mines: config.mines_count,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn remaining_mines(&self) -> u32 {
        let mut mines_count: u32 = 0;
        self.board.for_each_cell(|_, cell, _| {
            if cell.is_mine() && !(cell.cleared || cell.flagged) {
                mines_count += 1;
            }
        });

        mines_count
    }

    pub fn toggle_flagged(&mut self, coordinates: Point) {
        let Some(cell) = self.board.cell_at(coordinates) else {
            return;
        };

        if cell.cleared {
            return;
        }

        let new_cell = Cell { flagged: !cell.flagged, ..*cell };
        let mut m_cells = self.board.cells.clone();
        m_cells.replace_at(new_cell, cell.coordinates);
        self.board.cells = m_cells;
    }

    pub fn selected_at(&mut self, coordinates: Point) -> Cell {
        let mut selected_cell: Cell = Cell::default();
        Game::process_selected(&mut self.board, coordinates, &mut selected_cell);
        selected_cell
    }

    fn process_selected(board: &mut Board, coordinates: Point, selected_cell: &mut Cell) {
        if let Some(cell) = board.cell_at(coordinates).cloned() {
            *selected_cell = cell;

            Game::open_empty_cells(board, &cell);
        }
    }

    fn open_empty_cells(board: &mut Board, cell: &Cell) {
        let mut stack: Vec<Cell> = vec![];
        let mut checked_coordinates: Vec<Point> = vec![];

        stack.push(*cell);

        while !stack.is_empty() {
            match stack.pop() {
                Some(ref mut cell) => {
                    let point = cell.coordinates;
                    if checked_coordinates.contains(&point) {
                        continue;
                    }
                    checked_coordinates.push(point);

                    if cell.number == 0 {
                        let next_cells = board.get_cells_around(cell.coordinates);

                        next_cells.iter().for_each(|cell| {
                            if !checked_coordinates.contains(&cell.coordinates) {
                                stack.push(**cell);
                            }
                        });
                    }

                    let new_cell = Cell { cleared: true, ..*cell };
                    board.cells.replace_at(new_cell, cell.coordinates);
                }
                None => continue,
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        let mut is_game_over = false;
        self.board.for_each_cell(|_, cell, stop| {
            if cell.is_mine() && cell.cleared {
                is_game_over = true;
                *stop = true;
            }
        });
        is_game_over
    }

    pub fn is_win(&self) -> bool {
        if self.remaining_mines() > 0 {
            return false;
        }
        let mut is_win = true;
        self.board.for_each_cell(|_, cell, stop| {
            if cell.is_mine() && cell.cleared {
                is_win = false;
                *stop = true;
            }
        });
        is_win
    }

    pub fn clear_all_non_mines(&mut self) {
        self.board.for_each_cell_mut(|_, cell, _| {
            if !cell.cleared && !cell.is_mine() {
                cell.cleared = true;
            }
        });
    }

    pub fn clear_all(&mut self) {
        self.board.for_each_cell_mut(|_, cell, _| {
            if !cell.cleared {
                cell.cleared = true;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_gameover() {
        let mut game = Game::new(Difficulty::Easy);
        game.board.for_each_cell_mut(|_, cell, stop| {
            if cell.is_mine() {
                cell.cleared = true;
                *stop = true;
            }
        });
        assert!(game.is_game_over());
    }

    #[test]
    fn test_is_win() {
        let mut game = Game::new(Difficulty::Easy);
        game.board.for_each_cell_mut(|_, cell, _| {
            if cell.is_mine() {
                cell.flagged = true;
            }
        });
        assert!(game.is_win());
    }

    #[test]
    fn test_clear_white_cells() {
        let mut board = Board::new_empty(Size { width: 5, height: 5 });
        let mine_coordinates = Point { x: 2, y: 2 };
        board.replace_cell(Cell::new_mine(mine_coordinates), mine_coordinates);
        board.add_cell_numbers();

        let mut game = Game { board, total_mines: 1 };

        game.selected_at(Point { x: 0, y: 4 });

        println!(" ");

        print_board(&game.board);

        game.board.for_each_cell(|_, cell, _| {
            if cell.is_mine() {
                assert!(!cell.cleared);
            } else {
                assert!(cell.cleared);
            }
        });
    }

    fn print_board(board: &Board) {
        let mut last_row = 0;
        board.for_each_cell(|coordinates, cell, _| {
            if last_row != coordinates.x {
                last_row = coordinates.x;
                println!(" ");
            }
            let number = cell.number;
            if !cell.cleared {
                print!("[ \u{25AE}]");
            } else if number == -1 {
                print!("[ *]");
            } else if number == 0 {
                print!("[  ]");
            } else {
                print!("[ {}]", number);
            }
        });
        println!(" ");
    }
}
