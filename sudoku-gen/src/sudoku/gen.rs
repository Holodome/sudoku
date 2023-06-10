use super::sudoku::{Sudoku, SudokuSettings};
use bit_vec::BitVec;
use rand::Rng;

pub struct SudokuGenerator<Rand: Rng> {
    rng: Rand,
}

impl<Rand: Rng> SudokuGenerator<Rand> {
    pub fn new(rng: Rand) -> Self {
        Self { rng }
    }

    fn try_generate_filled(&mut self, settings: SudokuSettings) -> Option<Sudoku> {
        let mut sudoku = Sudoku::new(settings);
        for y in 0..sudoku.settings().height {
            for x in 0..sudoku.settings().width {
                let mut positions = BitVec::from_elem(sudoku.settings().value_count() + 1, false);
                for new_y in 0..sudoku.settings().height {
                    let value = sudoku.at(x, new_y) as usize;
                    positions.set(value, true);
                }
                for new_x in 0..sudoku.width() {
                    let value = sudoku.at(new_x, y) as usize;
                    positions.set(value, true);
                }
                let (subgrid_x, subgrid_y) = sudoku.settings().subgrid_start(x, y);
                for new_y in subgrid_y..subgrid_y + sudoku.settings().subgrid_height {
                    for new_x in subgrid_x..subgrid_x + sudoku.settings().subgrid_width {
                        let value = sudoku.at(new_x, new_y) as usize;
                        positions.set(value, true);
                    }
                }

                if positions.all() {
                    return None;
                }

                loop {
                    let value = self
                        .rng
                        .gen_range(sudoku.settings().min_value..=sudoku.settings().max_value);
                    if let Some(true) = positions.get(value as usize) {
                        continue;
                    }
                    *sudoku.at_mut(x, y) = value;
                    break;
                }
            }
        }

        Some(sudoku)
    }

    pub fn generate_filled(&mut self, settings: SudokuSettings) -> Sudoku {
        loop {
            if let Some(sudoku) = self.try_generate_filled(settings.clone()) {
                return sudoku;
            }
        }
    }
}
