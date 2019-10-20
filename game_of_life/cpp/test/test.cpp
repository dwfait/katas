#define CATCH_CONFIG_MAIN
#include "../src/gol.h"
#include "catch.hpp"

TEST_CASE("Creating a Game of Life board", "[gol]") {
  GOL game(1, 2);
  REQUIRE(game.get_x() == 1);
  REQUIRE(game.get_y() == 2);
}

TEST_CASE("Intial cell value is false", "[gol]") {
  GOL game(1, 2);
  REQUIRE(game.get_cell(0, 0) == false);
}

TEST_CASE("Manually setting cell state", "[gol]") {
  GOL game(1, 2);
  game.set_cell(0, 0, true);
  REQUIRE(game.get_cell(0, 0) == true);
}

TEST_CASE("1 alive neighbour -> dead cell", "[Rule 1]") {
  GOL game(2, 2);
  game.set_cell(0, 0, true);
  game.set_cell(0, 1, true);
  game.next_generation();
  REQUIRE(game.get_cell(0, 0) == false);
}
TEST_CASE("2 alive neighbours -> alive cell", "[Rule 1]") {
  GOL game(2, 2);
  game.set_cell(0, 0, true);
  game.set_cell(0, 1, true);
  game.set_cell(1, 0, true);
  game.next_generation();
  REQUIRE(game.get_cell(1, 0) == true);
}

TEST_CASE("3 alive neighbours -> alive cell", "[Rule 1]") {
  GOL game(2, 2);
  game.set_cell(0, 0, true);
  game.set_cell(0, 1, true);
  game.set_cell(1, 0, true);
  game.set_cell(1, 1, true);
  game.next_generation();
  REQUIRE(game.get_cell(1, 1) == true);
}
