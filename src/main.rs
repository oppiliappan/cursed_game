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

    fn live_neighbour_count() -> {
        let mut count = 0;
         
    }

    fn tick() -> Universe { }

}
