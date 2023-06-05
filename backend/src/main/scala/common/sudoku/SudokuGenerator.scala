package common.sudoku

import scala.annotation.tailrec
import scala.util.Random
import scala.collection.mutable.ArrayBuffer

object SudokuGenerator {
  def generateFilledBoard: MutableSudoku = {
    var board = new MutableSudoku(
      Array.fill(SudokuTraits.height)(
        Array.fill(SudokuTraits.width)(Option.empty[Int])
      )
    )

    def run(cellPositions: Set[(Int, Int)]): Boolean = {
      println(cellPositions.size)
      if (cellPositions.isEmpty) return true;

      for ((x, y) <- cellPositions) {
        val newPos = Random.shuffle(cellPositions - ((x, y)))
        val nums = Random.shuffle(getValidNumbersForPos(board, x, y))
        for (num <- nums) {
          board.cells(y)(x) = Some(num)
          if (run(newPos))
            return true;
        }

        board.cells(y)(x) = None;
      }

      false
    }

    val cellPositions = (for {
      y <- 0 until SudokuTraits.height
      x <- 0 until SudokuTraits.width
    } yield (x, y)).toSet

    run(cellPositions)
    board
  }

  def getValidNumbersForPos(
      sudoku: MutableSudoku,
      x: Int,
      y: Int
  ): List[Int] = {
    val row = sudoku.rows(y)
    val column = sudoku.columns(x)
    val (subgridX, subgridY) = SudokuTraits.posToSubgrid(x, y)
    val subgrid =
      sudoku
        .subgrids(subgridY * SudokuTraits.subgridHorizCount + subgridX)
        .flatten

    def cellSeqToValueSet(seq: Seq[SudokuTraits.Cell]): Set[Int] =
      seq.foldLeft(Set[Int]())((set, cell) =>
        cell match {
          case Some(value) => set + value
          case _           => set
        }
      )

    ((SudokuTraits.minCellValue to SudokuTraits.maxCellValue)
      .toSet[Int] -- cellSeqToValueSet(row) -- cellSeqToValueSet(
      column
    ) -- cellSeqToValueSet(subgrid)).toList
  }
}
