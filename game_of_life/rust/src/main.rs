fn main() {
    let mut g = Game::new(10, 10);

    // Set up a glider:
    g.set_cell(1, 2, true);
    g.set_cell(2, 2, true);
    g.set_cell(3, 2, true);
    g.set_cell(3, 1, true);
    g.set_cell(2, 0, true);

    for _i in 0..30 {
        g.new_generation();
        g.print_state();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

struct Board {
    board: Vec<Vec<bool>>,
}

struct Game {
    x: usize,
    y: usize,
    board: Board,
}

impl Board {
    fn new(x: usize, y: usize) -> Board {
        Board {
            board: vec![vec![false; y]; x],
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        self.board[x][y] = value;
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        return self.board[x][y];
    }
}

impl Game {
    fn new(x: usize, y: usize) -> Game {
        Game {
            x: x,
            y: y,
            board: Board::new(x, y),
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        self.board.set_cell(x, y, value);
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        return self.board.get_cell(x, y);
    }

    fn get_neighbourhood_count(&self, x: usize, y: usize) -> usize {
        let mut neighbourhood: usize = 0;
        let start_x: usize = std::cmp::max((x as i32) - 1, 0) as usize;
        let end_x: usize = std::cmp::min(x + 1, self.x - 1);
        let start_y: usize = std::cmp::max((y as i32) - 1, 0) as usize;
        let end_y: usize = std::cmp::min(y + 1, self.y - 1);

        for ix in start_x..=end_x {
            for iy in start_y..=end_y {
                if ix == x && iy == y {
                    continue;
                }
                if self.board.get_cell(ix, iy) {
                    neighbourhood += 1;
                }
            }
        }

        return neighbourhood;
    }

    fn new_generation(&mut self) {
        let mut new_board = Board::new(self.x, self.y);

        for x in 0..self.x {
            for y in 0..self.y {
                let neighbourhood = self.get_neighbourhood_count(x, y);

                if self.board.get_cell(x, y) {
                    if neighbourhood == 2 || neighbourhood == 3 {
                        new_board.set_cell(x, y, true);
                    } else {
                        new_board.set_cell(x, y, false);
                    }
                } else {
                    if neighbourhood == 3 {
                        new_board.set_cell(x, y, true);
                    }
                }
            }
        }

        self.board = new_board;
    }

    fn print_state(&self) {
        println!("Game state: ");
        for y in 0..self.y {
            for x in 0..self.x {
                if self.board.get_cell(x, y) {
                    print!("* ");
                } else {
                    print!(". ");
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new(10, 10);
        assert_eq!(game.x, 10);
    }

    #[test]
    fn test_setting_cell() {
        let mut game = Game::new(2, 2);
        assert_eq!(game.get_cell(1, 1), false);

        game.set_cell(1, 1, true);
        assert_eq!(game.get_cell(1, 1), true);
    }

    #[test]
    fn test_new_generation() {
        let mut game = Game::new(3, 3);

        game.set_cell(1, 1, true);
        game.set_cell(0, 1, true);
        game.set_cell(1, 0, true);

        assert_eq!(game.get_cell(0, 0), false);
        game.new_generation();
        assert_eq!(game.get_cell(0, 0), true);
    }

    #[test]
    fn test_rule_1() {
        // Fewer than 2 live neighbours, a cell dies
        let mut game = Game::new(3, 3);
        game.set_cell(0, 0, true);
        game.set_cell(0, 1, true);

        game.new_generation();
        assert_eq!(game.get_cell(0, 0), false);
    }

    #[test]
    fn test_rule_2() {
        // More than 3 live neighbours dies
        let mut game = Game::new(3, 3);
        game.set_cell(0, 0, true);
        game.set_cell(0, 1, true);
        game.set_cell(0, 2, true);
        game.set_cell(1, 0, true);
        game.set_cell(1, 1, true);
        game.set_cell(1, 2, true);
        game.set_cell(2, 0, true);
        game.set_cell(2, 1, true);
        game.set_cell(2, 2, true);

        game.new_generation();
        assert_eq!(game.get_cell(1, 1), false);
    }

    #[test]
    fn test_rule_3_2_live_neighbours() {
        // 2 live neighbours = live cell
        let mut game = Game::new(3, 3);
        game.set_cell(0, 0, true);
        game.set_cell(0, 1, true);
        game.set_cell(1, 0, true);

        game.new_generation();
        assert_eq!(game.get_cell(1, 0), true);
    }

    #[test]
    fn test_rule_3_3_live_neighbours() {
        // 2 live neighbours -> live cell
        let mut game = Game::new(3, 3);
        game.set_cell(0, 0, true);
        game.set_cell(0, 1, true);
        game.set_cell(0, 2, true);
        game.set_cell(1, 1, true);

        game.new_generation();
        assert_eq!(game.get_cell(1, 1), true);
    }

    #[test]
    fn test_rule_4() {
        // dead cell with exactly 3 -> live cell
        let mut game = Game::new(3, 3);
        game.set_cell(0, 0, true);
        game.set_cell(0, 1, true);
        game.set_cell(0, 2, true);

        game.new_generation();
        assert_eq!(game.get_cell(1, 1), true);
    }

    #[test]
    fn test_rule_4_more_than_3() {
        // dead cell with exactly 3 -> live cell
        let mut game = Game::new(3, 3);
        game.set_cell(0, 0, true);
        game.set_cell(0, 1, true);
        game.set_cell(0, 2, true);
        game.set_cell(2, 1, true);

        game.new_generation();
        assert_eq!(game.get_cell(1, 1), false);
    }
}
