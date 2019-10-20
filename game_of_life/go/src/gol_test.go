package gol

import "testing"

func TestNewBoard(t *testing.T) {
	board := NewBoard(1, 2)
	if board.x != 1 {
		t.Error("x must equal 1")
	}
	if board.y != 2 {
		t.Error("y must equal 2")
	}
}

func TestInitialCellValue(t *testing.T) {
	board := NewBoard(1, 1)
	if board.get_cell(0, 0) != false {
		t.Error("initial cell value must be false")
	}
}

func TestSettingValue(t *testing.T) {
	board := NewBoard(1, 1)
	board.set_cell(0, 0, true)
	if board.get_cell(0, 0) != true {
		t.Error("initial cell value must be false")
	}
}

func Test1AliveNeighbour(t *testing.T) {
	board := NewBoard(2, 2)

	board.set_cell(0, 0, true)
	board.set_cell(0, 1, true)

	board.next_generation()
	if board.get_cell(0, 1) != false {
		t.Error("cell with 1 alive neighbour must become dead")
	}
}

func Test2AliveNeighbours(t *testing.T) {
	board := NewBoard(2, 2)

	board.set_cell(0, 0, true)
	board.set_cell(0, 1, true)
	board.set_cell(1, 1, true)

	board.next_generation()
	if board.get_cell(0, 1) != true {
		t.Error("cell with 2 live neighbours must stay alive")
	}
}

func Test3AliveNeighbours(t *testing.T) {
	board := NewBoard(2, 2)

	board.set_cell(0, 0, true)
	board.set_cell(1, 0, true)
	board.set_cell(0, 1, true)
	board.set_cell(1, 1, true)

	board.next_generation()
	if board.get_cell(0, 1) != true {
		t.Error("cell with 3 live neighbours must stay alive")
	}
}
func Test4AliveNeighbours(t *testing.T) {
	board := NewBoard(3, 3)

	board.set_cell(0, 0, true)
	board.set_cell(1, 0, true)
	board.set_cell(0, 1, true)
	board.set_cell(1, 1, true)
	board.set_cell(2, 1, true)

	board.next_generation()
	if board.get_cell(1, 1) != false {
		t.Error("cell with 4 live neighbours must die")
	}
}

func TestDead3AliveNeighbours(t *testing.T) {
	board := NewBoard(3, 3)

	board.set_cell(0, 0, true)
	board.set_cell(1, 0, true)
	board.set_cell(0, 1, true)
	board.set_cell(1, 1, false)

	board.next_generation()
	if board.get_cell(1, 1) != true {
		t.Error("dead cell with 3 live neighbours must come alive")
	}
}

func TestDead4AliveNeighbours(t *testing.T) {
	board := NewBoard(3, 3)

	board.set_cell(0, 0, true)
	board.set_cell(1, 0, true)
	board.set_cell(0, 1, true)
	board.set_cell(1, 1, false)
	board.set_cell(2, 1, true)

	board.next_generation()
	if board.get_cell(1, 1) != false {
		t.Error("dead cell with 4 live neighbours must stay dead")
	}
}
