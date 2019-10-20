CXX := c++
CPP_FILES := $(wildcard src/*.cpp)
OBJ_FILES := $(addprefix obj/,$(notdir $(CPP_FILES:.cpp=.o)))
TEST_CPP_FILES := $(filter-out src/main.cpp, $(wildcard **/*.cpp))
TEST_OBJ_FILES := $(addprefix obj/,$(notdir $(TEST_CPP_FILES:.cpp=.o)))
CC_FLAGS := -Wall -Wextra -pedantic -std=c++2a

bin/gol: $(OBJ_FILES)
	$(CXX) -o $@ $^

bin/test: $(TEST_OBJ_FILES)
	$(CXX) -o $@ $^

obj/%.o: */%.cpp
	$(CXX) $(CC_FLAGS) -c -o $@ $<

test: bin/test
	bin/test

run: bin/gol
	bin/gol

clean:
	rm -f obj/*.o bin/*
