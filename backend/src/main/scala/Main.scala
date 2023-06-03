import cats.effect.{ExitCode, IO, IOApp}
import config.ServerConfig
import rest.Server

object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] = {
    val serverConfig = ServerConfig(8080)
    Server.run(serverConfig)
  }.map(_ => ExitCode.Success)
}
