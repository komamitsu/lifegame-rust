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

#[allow(dead_code)]
fn gen_test_cells() -> Cells {
    /*
          0123

        0 ##+#
        1 #+#+
        2 ++++
        3 +###
        4 ++#+
    */
    let mut cells = Cells::new(4, 5);
    for &(x, y) in [
        (0, 0), (1, 0), (3, 0),
        (0, 1), (2, 1),
        (1, 3), (2, 3), (3, 3),
        (2, 4)
    ].iter() {
        cells.set(x, y, Cell::Alive);
    }

    cells
}

#[test]
fn width_and_height() {
    let cells = gen_test_cells();
    assert_eq!(4, cells.width);
    assert_eq!(5, cells.height);
}

#[test]
fn set_and_get() {
    let mut cells = gen_test_cells();

    assert_eq!(Cell::Alive, *cells.get(0, 0));
    cells.set(0, 0, Cell::Dead);
    assert_eq!(Cell::Dead, *cells.get(0, 0));
    cells.set(0, 0, Cell::Alive);
    assert_eq!(Cell::Alive, *cells.get(0, 0));

    assert_eq!(Cell::Dead, *cells.get(3, 4));
    cells.set(3, 4, Cell::Alive);
    assert_eq!(Cell::Alive, *cells.get(3, 4));
}

#[test]
fn count_alive_adjacent_cells() {
    let cells = gen_test_cells();
    let expected = [
        [2, 3, 3, 1],
        [2, 4, 2, 2],
        [2, 4, 4, 3],
        [1, 2, 3, 2],
        [1, 3, 3, 3]
    ];
    for y in 0..cells.height {
        for x in 0..cells.width {
            let expected = expected[y][x];
            let actual = cells.count_alive_adjacent_cells(x, y);
            assert!(expected == actual,
                    "x={}, y={}, expected={}, actual={}",
                    x, y, expected, actual);
        }
    }
}

#[test]
fn step() {
    let mut cells = gen_test_cells();
    cells.step();

    use Cell::*;

    let expected = [
        [Alive, Alive, Alive, Dead],
        [Alive, Dead,  Alive, Dead],
        [Dead,  Dead,  Dead,  Alive],
        [Dead,  Alive, Alive, Alive],
        [Dead,  Alive, Alive, Alive]
    ];

    for y in 0..cells.height {
        for x in 0..cells.width {
            let expected = &expected[y][x];
            let actual = cells.get(x, y);
            assert!(*expected == *actual,
                    "x={}, y={}, expected={:?}, actual={:?}",
                    x, y, expected, actual);
        }
    }
}
