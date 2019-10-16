class GameOfLife():
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.cells = [[False for i in range(y)] for j in range(x)]

    def get_cell(self, x, y):
        return self.cells[x][y]

    def set_cell(self, x, y, value):
        self.cells[x][y] = value

    def get_neighbourhood(self, x, y):
        neighbourhood = 0
        for ix in range(max(x - 1, 0), min(self.x, x + 2)):
            for iy in range(max(0, y - 1), min(self.y, y + 2)):
                if ix == x and iy == y:
                    continue
                if self.cells[ix][iy] == True:
                    neighbourhood += 1
        return neighbourhood

    def next_generation(self):
        self.cells[0][0] = False

        new_cells = [[False for i in range(self.y)] for j in range(self.x)]

        for x in range(self.x):
            for y in range(self.y):
                neighbourhood = self.get_neighbourhood(x, y)

                if neighbourhood < 2:
                    new_cells[x][y] = False
                else:
                    new_cells[x][y] = True

        self.cells = new_cells
