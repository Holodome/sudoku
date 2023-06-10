mod rng;
mod sudoku;

use rand::SeedableRng;
use sudoku::sudoku::SudokuSettings;

fn main() {
    let entropy = rng::Xorshift32::from_entropy();
    let mut gen = sudoku::gen::SudokuGenerator::new(entropy);
    let settings = SudokuSettings {
        min_value: 1,
        max_value: 9,
        width: 9,
        height: 9,
        subgrid_width: 3,
        subgrid_height: 3,
        subgrid_vert_count: 3,
        subgrid_horiz_count: 3,
    };
    let board = gen.generate_filled(settings);
    println!("{:#?}", board);
}
