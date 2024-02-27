use solver::{solve, Pos};

mod solver;

fn main() {
    for (i, solution) in solve(Pos { x: 6, y: 6 }, Pos { x: 0, y: 0 }).enumerate() {
        println!("Solution {}", i + 1);
        for step in solution {
            println!("{:?}", step);
        }
        println!();
    }
}
