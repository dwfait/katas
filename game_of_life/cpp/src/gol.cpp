#include "gol.h"
#include <algorithm>

GOL::GOL(size_t x, size_t y) : x{x}, y{y}, cells(nullptr) {
  this->cells = this->init_cells();
}

size_t GOL::get_x() { return this->x; }
size_t GOL::get_y() { return this->y; }

bool GOL::get_cell(size_t x, size_t y) { return (*this->cells)[x][y]; }

void GOL::set_cell(size_t x, size_t y, Cell value) {
  (*this->cells)[x][y] = value;
}

CellsPtr GOL::init_cells() {
  return std::make_unique<Cells>(this->x, CellRow(this->y, false));
}

void GOL::next_generation() {
  CellsPtr new_cells = std::move(this->init_cells());

  for (size_t ix = 0; ix < this->x; ++ix) {
    for (size_t iy = 0; iy < this->y; ++iy) {
      auto neighbourhood = this->get_neighbourhood(ix, iy);

      if (neighbourhood >= 2)
        (*new_cells)[ix][iy] = true;
    }
  }

  this->cells = std::move(new_cells);
}

size_t GOL::get_neighbourhood(size_t x, size_t y) {
  size_t neighbourhood = 0;
  size_t start_x = std::max(0, int(x - 1));
  size_t end_x = std::min((int)this->x - 1, int(x + 1));

  size_t start_y = std::max(0, int(y - 1));
  size_t end_y = std::min((int)this->y - 1, int(y + 1));

  for (size_t ix = start_x; ix <= end_x; ++ix) {
    for (size_t iy = start_y; iy <= end_y; ++iy) {

      if (ix == x && iy == y)
        continue;
      if (this->get_cell(ix, iy) == true) {
        ++neighbourhood;
      }
    }
  }

  return neighbourhood;
}
