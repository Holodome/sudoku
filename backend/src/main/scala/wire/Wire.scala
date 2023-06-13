package wire

import io.grpc.ManagedChannelBuilder
import wire.sudoku.{
  ClassicalSudokuBoard,
  ClassicalSudokuBoardSettings,
  SudokuGrpc
}

import scala.concurrent.Future

class Wire(val host: String, val port: Int) {
  private val channel =
    ManagedChannelBuilder.forAddress(host, port).usePlaintext().build
  private val grpcStub = SudokuGrpc.blockingStub(channel)

  def getBoard(
      classicalSudokuBoardSettings: ClassicalSudokuBoardSettings
  ): ClassicalSudokuBoard =
    grpcStub.generateFilled(classicalSudokuBoardSettings)
}
