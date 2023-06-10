#[derive(Clone, Debug)]
pub struct SudokuSettings {
    pub min_value: u8,
    pub max_value: u8,
    pub width: usize,
    pub height: usize,
    pub subgrid_width: usize,
    pub subgrid_height: usize,
    pub subgrid_horiz_count: usize,
    pub subgrid_vert_count: usize,
}

impl SudokuSettings {
    pub fn value_count(&self) -> usize {
        (self.max_value - self.min_value + 1) as usize
    }

    pub fn subgrid_start(&self, x: usize, y: usize) -> (usize, usize) {
        (
            x / self.subgrid_width * self.subgrid_width,
            y / self.subgrid_height * self.subgrid_height,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Sudoku {
    settings: SudokuSettings,
    tiles: Vec<u8>,
}

impl Sudoku {
    pub fn new(settings: SudokuSettings) -> Self {
        let size = settings.width * settings.height;
        Self {
            settings,
            tiles: vec![0; size],
        }
    }

    pub fn settings(&self) -> &SudokuSettings {
        &self.settings
    }

    pub fn width(&self) -> usize {
        self.settings.width
    }
    pub fn height(&self) -> usize {
        self.settings.height
    }

    pub fn at(&self, x: usize, y: usize) -> u8 {
        self.tiles[y * self.settings.width + x]
    }
    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.tiles[y * self.settings.width + x]
    }
}
