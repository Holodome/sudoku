package common.sudoku

object SudokuTraits {
  type Cell = Option[Int]

  def minCellValue: Int = 1
  def maxCellValue: Int = 9

  def width: Int = 9
  def height: Int = 9

  def subgridWidth: Int = 3
  def subgridHeight: Int = 3

  def subgridHorizCount: Int = 3
  def subgridVertCount: Int = 3

  def isSeqSolved(seq: Seq[Cell]): Boolean =
    (minCellValue to maxCellValue).forall(v =>
      seq.exists(_.exists(it => it == v))
    )

  def posToSubgrid(x: Int, y: Int): (Int, Int) =
    (x / subgridWidth, y / subgridHeight)
}