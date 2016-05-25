extern crate rand;
use std::thread;
use std::time::Duration;

mod lifegame {
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Cell {
        Dead,
        Alive
    }

    pub struct Cells {
        pub width: usize,
        pub height: usize,
        cells: Vec<Vec<Cell>>
    }

    impl Cells {
        pub fn new(width: usize, height: usize) -> Cells {
            let mut cells = vec![];
            for _ in 0..height {
                let mut row = vec![];
                for _ in 0..width {
                    row.push(Cell::Dead);
                }
                cells.push(row);
            }

            Cells { height: height, width: width, cells: cells }
        }

        #[allow(dead_code)]
        pub fn get(&self, x: usize, y: usize) -> &Cell {
            &self.cells[y][x]
        }

        pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
            self.cells[y][x] = cell;
        }

        fn count_alive_cell_in_row(&self, row: &Vec<Cell>, pos: usize, skip_same_pos: bool) -> i32 {
            let mut count = 0;
            if pos > 0 && row[pos - 1] == Cell::Alive {
                count += 1;
            }
            if !skip_same_pos && row[pos] == Cell::Alive {
                count += 1;
            }
            if pos < self.width - 1 && row[pos + 1] == Cell::Alive {
                count += 1;
            }
            count
        }

        pub fn count_alive_adjacent_cells(&self, x: usize, y: usize) -> i32 {
            let mut alive_cells = 0;
            if y > 0 {
                alive_cells += self.count_alive_cell_in_row(&self.cells[y - 1], x, false);
            }
            alive_cells += self.count_alive_cell_in_row(&self.cells[y], x, true);
            if y < self.height - 1 {
                alive_cells += self.count_alive_cell_in_row(&self.cells[y + 1], x, false);
            }
            alive_cells
        }

        pub fn step(&mut self) {
            let mut updated_cells = vec![];

            for y in 0..self.height {
                let mut updated_row = vec![];

                for x in 0..self.width {
                    let adjacent_cells = self.count_alive_adjacent_cells(x, y);
                    let next_cell = 
                        match self.cells[y][x] {
                            Cell::Alive => match adjacent_cells {
                                2 | 3 => Cell::Alive,
                                0 | 1 => Cell::Dead,
                                _ => Cell::Dead
                            },
                            Cell::Dead => match adjacent_cells {
                                3 => Cell::Alive,
                                _ => Cell::Dead
                            }
                        };
                    updated_row.push(next_cell);
                }
                updated_cells.push(updated_row);
            }

            self.cells = updated_cells;
        }

        pub fn print(&self) {
            for row in &self.cells {
                for cell in row {
                    match cell {
                        &Cell::Alive => print!("#"),
                        &Cell::Dead => print!(".")
                    }
                }
                println!("")
            }
        }
    }
}

fn main() {
    let mut cells = lifegame::Cells::new(20, 20);
    for _ in 0..50 {
        let x = rand::random::<usize>() % cells.width;
        let y = rand::random::<usize>() % cells.height;
        cells.set(x, y, lifegame::Cell::Alive);
    }

    loop {
        cells.print();
        println!("");
        cells.step();
        thread::sleep(Duration::from_secs(1));
    }
}
