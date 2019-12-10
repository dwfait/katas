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

  public void testSettingCellValue() {
    GameOfLife gol = new GameOfLife(1, 1);
    assertFalse(gol.get_cell(0, 0));
    gol.set_cell(0, 0, true);
    assertTrue(gol.get_cell(0, 0));
  }

  public void testRuleOne() {
    GameOfLife gol = new GameOfLife(2, 2);
    gol.set_cell(0, 0, true);
    gol.next_generation();
    assertFalse(gol.get_cell(0, 0));
  }

  public void testRuleTwoMoreThan3Neighbours() {
    GameOfLife gol = new GameOfLife(3, 3);
    gol.set_cell(0, 0, true);
    gol.set_cell(1, 0, true);
    gol.set_cell(0, 1, true);
    gol.set_cell(1, 1, true);
    gol.set_cell(1, 2, true);
    gol.next_generation();
    assertFalse(gol.get_cell(1, 1));
  }

  public void testRuleThreeTwoNeighbours() {
    GameOfLife gol = new GameOfLife(2, 2);
    gol.set_cell(0, 0, true);
    gol.set_cell(1, 0, true);
    gol.set_cell(0, 1, true);
    gol.next_generation();
    assertTrue(gol.get_cell(0, 0));
  }

  public void RuleFourDeadWithThreeNeighbours() {
    GameOfLife gol = new GameOfLife(2, 2);
    gol.set_cell(0, 0, true);
    gol.set_cell(1, 0, true);
    gol.set_cell(0, 1, true);
    gol.set_cell(1, 1, false);
    gol.next_generation();
    assertTrue(gol.get_cell(1, 1));
  }
}
