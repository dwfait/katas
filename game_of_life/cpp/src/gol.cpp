#include "gol.h"

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
  return CellsPtr(new Cells(this->x, CellRow(this->y, false)));
}

void GOL::next_generation() {
  for (size_t ix = 0; ix < this->x; ++ix) {
    for (size_t iy = 0; iy < this->y; ++iy) {
      (*this->cells)[ix][iy] = false;
    }
  }
}
