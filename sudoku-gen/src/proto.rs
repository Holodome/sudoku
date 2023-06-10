use tonic::{Request, Response, Status};

pub mod sudoku {
    tonic::include_proto!("sudoku");
}

#[derive(Default)]
pub struct SudokuService {}

#[tonic::async_trait]
impl sudoku::sudoku_server::Sudoku for SudokuService {
    async fn generate_filled(
        &self,
        request: Request<sudoku::ClassicalSudokuBoardSettings>,
    ) -> Result<Response<sudoku::ClassicalSudokuBoard>, Status> {
        let reply = sudoku::ClassicalSudokuBoard { board: vec![] };
        Ok(Response::new(reply))
    }
}
