use solver::{solve, Pos};

mod solver;

fn main() {
    for (i, solution) in solve(Pos { x: 5, y: 5 }, Pos { x: 0, y: 0 })
        .iter()
        .enumerate()
    {
        println!("Solution {}", i + 1);
        for step in solution {
            println!("{:?}", step);
        }
        println!();
    }
}
