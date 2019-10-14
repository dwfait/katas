class GameOfLife():
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.cells = [[False for i in range(y)] for j in range(x)]

    def get_cell(self, x, y):
        return self.cells[x][y]

    def set_cell(self, x, y, value):
        self.cells[x][y] = value

