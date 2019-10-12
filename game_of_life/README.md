# Conway's Game of Life

A simple cellular automata, which I find a nice kata to try out new languages and techniques on.

Simple enough to be implemented fairly quickly, with enough scope for a variety of organisational methods and implementation techniques.

## Rules

   1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
   2. Any live cell with more than three live neighbours dies, as if by overcrowding.
   3. Any live cell with two or three live neighbours lives on to the next generation.
   4. Any dead cell with exactly three live neighbours becomes a live cell.

## Example

Generation 1:
4 8
........
....*...
...**...
........

Generation 2:
4 8
........
...**...
...**...
........
