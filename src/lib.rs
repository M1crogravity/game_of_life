pub struct Universe {
    cells: [[u8; 25]; 25],
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            cells: [[0; 25]; 25],
        }
    }

    pub fn glider(&mut self) {
        self.cells[13][12] = 1;
        self.cells[13][13] = 1;
        self.cells[13][14] = 1;
        self.cells[12][14] = 1;
        self.cells[11][13] = 1;
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for i in 0..25 {
            for j in 0..25 {
                let cell = self.cells[i][j];
                let live_neighbors = self.live_neighbors(i, j);
                match (cell, live_neighbors) {
                    (0, 3) => next[i][j] = 1,
                    (1, 2) | (1, 3) => next[i][j] = 1,
                    (_, _) => next[i][j] = 0,
                }
            }
        }

        self.cells = next;
    }

    fn live_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for i in row.checked_sub(1).unwrap_or(0)..(row + 2).min(24) {
            for j in col.checked_sub(1).unwrap_or(0)..(col + 2).min(24) {
                if i == row && j == col {
                    continue;
                }
                count += self.cells[i][j];
            }
        }
        count
    }

    pub fn display(&self) {
        for i in 0..25 {
            for j in 0..25 {
                print!("[{}]", self.cells[i][j]);
            }
            println!("");
        }
    }
}
