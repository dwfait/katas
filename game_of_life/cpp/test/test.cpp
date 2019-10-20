#define CATCH_CONFIG_MAIN  // This tells Catch to provide a main() - only do this in one cpp file
#include "catch.hpp"
#include "../src/gol.h"

TEST_CASE("Blah blah blah", "blah") {
  REQUIRE(test_fun() == 42);
}
