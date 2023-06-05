package common.sudoku

class MutableSudoku(var cells: MutableSudoku.Subgrid) {
  import SudokuTraits._
  import MutableSudoku._

  assert(cells.length == 9);
  assert(cells.head.length == 9);

  def columns: Array[Array[Cell]] =
    (0 until width).map(i => cells.map(_(i))).toArray
  def rows: Array[Array[Cell]] = cells
  def subgrids: Array[Subgrid] = {
    for {
      sy <- 0 until subgridVertCount
      sx <- 0 until subgridHorizCount
    } yield cells.slice(sy, sy + subgridHeight).slice(sx, sx + subgridWidth)
  }.toArray

  def isSolved: Boolean =
    rows.forall(s => isSeqSolved(s.toSeq)) && columns.forall(s =>
      isSeqSolved(s.toSeq)
    ) && subgrids.forall(s => isSeqSolved(s.flatten.toSeq))
}

object MutableSudoku {
  import SudokuTraits.Cell
  type Subgrid = Array[Array[Cell]]
}
