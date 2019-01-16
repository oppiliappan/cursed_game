use std::fmt;

#[derive(Clone, Copy)]
enum Cell {
    Dead = 0,
    Alive = 1
}

#[derive(Clone)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl Universe {

    pub fn new(width: u32, height: u32) -> Universe {
        let length = width * height;
        let cells: Vec<Cell> = (0..length).map(|i| {
            //if i % 3 == 0 || i % 7 == 0 {
            //    Cell::Alive
            //} else  {
            //    Cell::Dead
            //}
            if i % width == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        Universe {
            width,
            height,
            cells
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u32 {
        let mut count = 0;
        for del_row in [self.height - 1, 0, 1].iter().cloned() {
            for del_col in [self.width - 1, 0, 1].iter().cloned() {
                if del_row == 0 && del_col == 0 {
                    continue;
                }
                let neighbour_row = ( row + del_row ) % self.height;
                let neighbour_col = ( col + del_row ) % self.height;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u32;
            }
        }
        count
    }

    pub fn tick(&mut self) -> String {
        let mut new = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbour_count = self.live_neighbour_count(row, col);

                let new_cell = match (cell, live_neighbour_count) { 
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (x, _) => x
                };

                new[index] = new_cell;
            }
        }
        self.cells = new;
        format!("{}", self)
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        for row in self.cells.as_slice().chunks(self.width as usize) {
            for cell in row {
                match cell {
                    Cell::Dead => write!(f, " □ "),
                    Cell::Alive => write!(f, " ■ "),
                }.unwrap();
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
