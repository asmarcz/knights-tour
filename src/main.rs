use std::sync::mpsc::{self, Receiver, Sender};

use solver::{solve, Pos};

mod solver;

fn main() {
    let (tx, rx): (Sender<Vec<Pos>>, Receiver<Vec<Pos>>) = mpsc::channel();
    std::thread::spawn(move || {
        solve(Pos { x: 6, y: 6 }, Pos { x: 0, y: 0 }, tx);
    });

    let mut sol_cnt = 0;
    while let Ok(solution) = rx.recv() {
        sol_cnt += 1;
        println!("Solution {}", sol_cnt);
        for step in solution {
            println!("{:?}", step);
        }
    }
}
