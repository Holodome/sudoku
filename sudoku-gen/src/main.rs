mod wire;
mod rng;
mod sudoku;

use wire::SudokuService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8080".parse().unwrap();
    let sudoku_service = SudokuService::new();

    Server::builder()
        .add_service(wire::sudoku::sudoku_server::SudokuServer::new(
            sudoku_service,
        ))
        .serve(addr)
        .await?;
    Ok(())
}
