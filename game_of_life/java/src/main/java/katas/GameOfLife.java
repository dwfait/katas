package katas;

public class GameOfLife {
  int x, y;

  public GameOfLife(int x, int y) {
    this.x = x;
    this.y = y;
  }

  public int get_x() {
    return this.x;
  }

  public int get_y() {
    return this.y;
  }

  public boolean get_cell(int x, int y) {
    return false;
  }
}
