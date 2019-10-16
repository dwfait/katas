import unittest
import game_of_life


class TestGameOfLife(unittest.TestCase):
    def test_board_initialisation(self):
        game_of_life.GameOfLife(10, 10)

    def test_initial_cell_state_is_dead(self):
        board = game_of_life.GameOfLife(1, 1)
        self.assertEqual(board.get_cell(0, 0), False)

    def test_cell_can_be_stimulated(self):
        board = game_of_life.GameOfLife(1, 1)
        board.set_cell(0, 0, True)
        self.assertEqual(board.get_cell(0, 0), True)

    def test_larger_board_stimulating(self):
        board = game_of_life.GameOfLife(5, 5)
        board.set_cell(4, 3, True)
        self.assertEqual(board.get_cell(4, 3), True)
        self.assertEqual(board.get_cell(3, 3), False)

    def test_rule1_live_cell_becomes_dead(self):
        board = game_of_life.GameOfLife(1, 1)
        board.set_cell(0, 0, True)
        board.next_generation()
        self.assertEqual(board.get_cell(0, 0), False)

    def test_rule1_live_cell_with_2_neighbours_stays_alive(self):
        board = game_of_life.GameOfLife(2, 2)
        board.set_cell(0, 0, True)
        board.set_cell(0, 1, True)
        board.set_cell(1, 0, True)
        board.next_generation()
        self.assertEqual(board.get_cell(0, 0), True)


if __name__ == '__main__':
    unittest.main()
