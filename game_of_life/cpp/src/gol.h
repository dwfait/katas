#pragma once

#include "stddef.h"

class GOL {
  size_t x, y;

public:
  GOL(size_t x, size_t y) : x{x}, y{y} {}

  size_t get_x();
  size_t get_y();
};
