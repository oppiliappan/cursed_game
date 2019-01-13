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

    fn live_neighbour_count(&self, row: u32, col: u32) -> u32 {
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

    fn tick() -> Universe { }

}
