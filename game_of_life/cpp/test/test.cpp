#define CATCH_CONFIG_MAIN
#include "../src/gol.h"
#include "catch.hpp"

TEST_CASE("Creating a Game of Life board", "[gol]") {
  GOL game(1, 2);
  REQUIRE(game.get_x() == 1);
  REQUIRE(game.get_y() == 2);
}
