fn main() {
    println!("Hello, world!");
}

enum Cell {
    Dead = 0,
    Alive = 1
}

struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl Universe {

    fn new() -> Universe { }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for del_row in [self.height - 1, 0, 1].iter().cloned() {
            for del_col in [self.width - 1, 0, 1].iter().cloned() {
                if del_row == 0 && del_col == 0 {
                    continue;
                }
                let neighbour_row = ( row + del_row ) % self.height;
                let neighbour_col = ( col + del_row ) % self.height;
            }
        }
    }

    fn tick(&mut cells) -> Universe {
        let mut new = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbour_count = self.live_neighbour_count(row, col);

                match (cell, live_neighbour_count) { 
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive
                };

                new[index] = cell;
            }
        }
    }
    self.cells = new;
}
