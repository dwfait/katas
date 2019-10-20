package gol

type Board struct {
	x     int
	y     int
	cells *[][]bool
}

// Golang only has a built in min/max for floats.
// Lots of people say Go doesn't need generics.
// I disagree.
func min(x, y int) int {
	if x < y {
		return x
	}
	return y
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}

func init_board(x int, y int) *[][]bool {
	var cells = make([][]bool, x)
	for i := range cells {
		cells[i] = make([]bool, y)
	}
	return &cells
}

func NewBoard(x int, y int) *Board {
	return &Board{x: x, y: y, cells: init_board(x, y)}
}

func (b *Board) get_cell(x int, y int) bool {
	return (*b.cells)[x][y]
}

func (b *Board) set_cell(x int, y int, value bool) {
	(*b.cells)[x][y] = value
}

func (b *Board) next_generation() {
	var new_cells = init_board(b.x, b.y)

	for ix, row := range *b.cells {
		for iy, cell := range row {
			var neighbourhood = b.get_neighbourhood(ix, iy)

			if cell == true {
				if neighbourhood == 2 || neighbourhood == 3 {
					(*new_cells)[ix][iy] = true
				}
			} else {
				if neighbourhood == 3 {
					(*new_cells)[ix][iy] = true
				}
			}
		}
	}
	b.cells = new_cells
}

func (b *Board) get_neighbourhood(x int, y int) int {
	var neighbourhood = 0
	var start_x = max(0, x-1)
	var end_x = min(b.x-1, x+1)

	var start_y = max(0, y-1)
	var end_y = min(b.y-1, y+1)

	for ix := start_x; ix <= end_x; ix += 1 {
		for iy := start_y; iy <= end_y; iy += 1 {
			if ix == x && iy == y {
				continue
			}

			if b.get_cell(ix, iy) == true {
				neighbourhood++
			}
		}
	}
	return neighbourhood
}
