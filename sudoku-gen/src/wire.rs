use crate::rng::Xorshift32;
use crate::sudoku::gen::SudokuGenerator;
use crate::sudoku::sudoku::SudokuSettings;
use rand::SeedableRng;
use std::sync::Arc;
use std::sync::Mutex;
use tonic::{Request, Response, Status};

pub mod sudoku {
    tonic::include_proto!("sudoku");
}

pub struct SudokuService {
    sudoku_gen: Arc<Mutex<SudokuGenerator<Xorshift32>>>,
}

impl SudokuService {
    pub fn new() -> Self {
        let rng = Xorshift32::from_entropy();
        let sudoku_gen = Arc::new(Mutex::new(SudokuGenerator::new(rng)));
        Self { sudoku_gen }
    }
}

#[tonic::async_trait]
impl sudoku::sudoku_server::Sudoku for SudokuService {
    async fn generate_filled(
        &self,
        request: Request<sudoku::ClassicalSudokuBoardSettings>,
    ) -> Result<Response<sudoku::ClassicalSudokuBoard>, Status> {
        let settings = request.get_ref();
        let actual_settings = SudokuSettings {
            min_value: settings.min_value as u8,
            max_value: settings.max_value as u8,
            width: settings.width as usize,
            height: settings.height as usize,
            subgrid_width: settings.subgrid_width as usize,
            subgrid_height: settings.subgrid_height as usize,
            subgrid_horiz_count: settings.subgrid_horiz_count as usize,
            subgrid_vert_count: settings.subgrid_vert_count as usize,
        };
        let board = {
            let mut lock = self.sudoku_gen.lock().map_err(|_| Status::aborted(""))?;
            lock.generate_filled(actual_settings)
        };
        let reply = sudoku::ClassicalSudokuBoard {
            board: Vec::from(board.tiles()),
        };
        Ok(Response::new(reply))
    }
}
