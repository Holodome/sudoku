use proto::SudokuService;
use tonic::transport::Server;

mod proto;
mod rng;
mod sudoku;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8080".parse().unwrap();
    let sudoku_service = SudokuService::new();

    Server::builder()
        .add_service(proto::sudoku::sudoku_server::SudokuServer::new(
            sudoku_service,
        ))
        .serve(addr)
        .await?;
    Ok(())
}
