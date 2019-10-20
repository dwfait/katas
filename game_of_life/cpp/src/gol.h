#pragma once

#include "stddef.h"
#include <memory>
#include <vector>

using std::vector;

typedef bool Cell;
typedef vector<Cell> CellRow;
typedef vector<CellRow> Cells;
typedef std::unique_ptr<Cells> CellsPtr;

class GOL {
  size_t x, y;
  CellsPtr cells;

public:
  GOL(size_t x, size_t y);
  size_t get_x();
  size_t get_y();

  Cell get_cell(size_t x, size_t y);
  void set_cell(size_t x, size_t y, Cell value);

  void next_generation();

private:
  CellsPtr init_cells();
};
