import unittest
import game_of_life

class TestGameOfLife(unittest.TestCase):
    def test_board_initialisation(self):
        game_of_life.GameOfLife(10, 10)

if __name__ == '__main__':
  unittest.main()

