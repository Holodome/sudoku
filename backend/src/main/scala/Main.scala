import cats.effect.{ExitCode, IO, IOApp}
import config.ServerConfig
import rest.Server
import wire.Wire
import wire.sudoku.ClassicalSudokuBoardSettings

import scala.concurrent.ExecutionContext

object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] = {
    val wire = new Wire("localhost", 8080)
    val settings = new ClassicalSudokuBoardSettings(1, 9, 9, 9, 3, 3, 3, 3)
    implicit val ec = ExecutionContext.global
    println("here")
    val board = wire.getBoard(settings)
    println(board)

    val serverConfig = ServerConfig(1234)
    Server.run(serverConfig)
  }.map(_ => ExitCode.Success)
}
