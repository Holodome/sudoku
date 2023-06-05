import cats.effect.{ExitCode, IO, IOApp}
import common.sudoku.SudokuGenerator
import config.ServerConfig
import rest.Server

object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] = {
    val sudoku = SudokuGenerator.generateFilledBoard
    for (
      row <- sudoku.cells
      ){
      for ( it <- row) {
        print(it)
        print(" ")
      }
      println("")
    }

    while( true){

    }

    val serverConfig = ServerConfig(8080)
    Server.run(serverConfig)
  }.map(_ => ExitCode.Success)
}
