extern crate rand;
extern crate lifegame;
use std::thread;
use std::time::Duration;

fn main() {
    let mut cells = lifegame::Cells::new(80, 40);
    for _ in 0..500 {
        let x = rand::random::<usize>() % cells.width;
        let y = rand::random::<usize>() % cells.height;
        cells.set(x, y, lifegame::Cell::Alive);
    }

    loop {
        cells.print();
        println!("");
        cells.step();
        thread::sleep(Duration::from_millis(200));
    }
}
