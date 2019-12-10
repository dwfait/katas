package katas;

import java.util.ArrayList;

public class GameOfLife {
  private ArrayList<ArrayList<Boolean>> cells;
  private int x, y;

  public GameOfLife(int x, int y) {
    this.x = x;
    this.y = y;
    this.cells = create_cells();
  }

  public int get_x() {
    return this.x;
  }

  public int get_y() {
    return this.y;
  }

  public boolean get_cell(int x, int y) {
    return this.cells.get(x).get(y);
  }

  public void set_cell(int x, int y, boolean value) {
    this.cells.get(x).set(y, value);
  }

  public void next_generation() {
    ArrayList<ArrayList<Boolean>> new_cells = create_cells();

    for (int ix = 0; ix < this.get_x(); ix++) {
      for (int iy = 0; iy < this.get_y(); iy++) {
        int neighbourhood = get_neighbourhood(ix, iy);

        if (get_cell(ix, iy)) {
          if (neighbourhood == 3 || neighbourhood == 2) {
            new_cells.get(ix).set(iy, true);
          }
        } else {
          if (neighbourhood == 3) {
            new_cells.get(ix).set(iy, true);
          }
        }
      }
    }

    this.cells = new_cells;
  }

  private int get_neighbourhood(int x, int y) {
    int neighbourhood = 0;

    for (int ix = (Math.max(x - 1, 0)); ix <= Math.min(x + 1, this.get_x() - 1); ix++) {
      for (int iy = (Math.max(y - 1, 0)); iy <= Math.min(y + 1, this.get_y() - 1); iy++) {

        if (ix == x && iy == y) {
          continue;
        }

        if (this.get_cell(ix, iy)) {
          neighbourhood += 1;
        }
      }
    }
    return neighbourhood;
  }

  private ArrayList<ArrayList<Boolean>> create_cells() {
    ArrayList<ArrayList<Boolean>> cells = new ArrayList<ArrayList<Boolean>>();
    for (int i = 0; i < this.get_x(); i++) {
      ArrayList<Boolean> row = new ArrayList<Boolean>();
      for (int j = 0; j < this.get_y(); j++) {
        row.add(false);
      }
      cells.add(row);
    }
    return cells;
  }
}
