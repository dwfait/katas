package katas;

import junit.framework.Test;
import junit.framework.TestCase;
import junit.framework.TestSuite;

public class GameOfLifeTest extends TestCase {
  public GameOfLifeTest(String testName) {
    super(testName);
  }

  public static Test suite() {
    return new TestSuite(GameOfLifeTest.class);
  }

  public void testNewBoard() {
    GameOfLife gol = new GameOfLife(1, 2);
    assertTrue(gol.get_x() == 1);
    assertTrue(gol.get_y() == 2);
  }

  public void testInitialCellValue() {
    GameOfLife gol = new GameOfLife(1, 1);
    assertFalse(gol.get_cell(0, 0));
  }
}
