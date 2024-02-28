use std::sync::mpsc::Sender;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

fn add_jumps(dimensions: Pos, from_pos: Pos, acc: &mut Vec<Pos>) {
    // to the right
    if from_pos.x + 2 <= dimensions.x - 1 {
        if from_pos.y + 1 <= dimensions.y - 1 {
            acc.push(Pos {
                x: from_pos.x + 2,
                y: from_pos.y + 1,
            })
        }
        if from_pos.y >= 1 {
            acc.push(Pos {
                x: from_pos.x + 2,
                y: from_pos.y - 1,
            });
        }
    }

    // to the left
    if from_pos.x >= 2 {
        if from_pos.y + 1 <= dimensions.y - 1 {
            acc.push(Pos {
                x: from_pos.x - 2,
                y: from_pos.y + 1,
            })
        }
        if from_pos.y >= 1 {
            acc.push(Pos {
                x: from_pos.x - 2,
                y: from_pos.y - 1,
            });
        }
    }

    // down
    if from_pos.y + 2 <= dimensions.y - 1 {
        if from_pos.x + 1 <= dimensions.x - 1 {
            acc.push(Pos {
                x: from_pos.x + 1,
                y: from_pos.y + 2,
            });
        }
        if from_pos.x >= 1 {
            acc.push(Pos {
                x: from_pos.x - 1,
                y: from_pos.y + 2,
            });
        }
    }

    // up
    if from_pos.y >= 2 {
        if from_pos.x + 1 <= dimensions.x - 1 {
            acc.push(Pos {
                x: from_pos.x + 1,
                y: from_pos.y - 2,
            });
        }
        if from_pos.x >= 1 {
            acc.push(Pos {
                x: from_pos.x - 1,
                y: from_pos.y - 2,
            });
        }
    }
}

fn solve_branch(
    dimensions: Pos,
    sender: Sender<Vec<Pos>>,
    mut board: Vec<Vec<bool>>,
    mut stack: Vec<(Pos, bool)>,
    mut visited_sq_cnt: usize,
    target_sq_cnt: usize,
) {
    let mut jump_acc = vec![];
    while !stack.is_empty() {
        let (curr_pos, should_close) = stack.pop().unwrap();
        let curr_square = unsafe {
            board
                .get_unchecked_mut(curr_pos.x)
                .get_unchecked_mut(curr_pos.y)
        };
        if should_close {
            *curr_square = false;
            visited_sq_cnt -= 1;
            continue;
        }
        *curr_square = true;
        visited_sq_cnt += 1;
        stack.push((curr_pos, true));

        if visited_sq_cnt == target_sq_cnt {
            sender
                .send(stack.iter().filter(|p| p.1).map(|p| p.0).collect())
                .unwrap();
        }

        jump_acc.truncate(0);
        add_jumps(dimensions, curr_pos, &mut jump_acc);
        for next_pos in &jump_acc {
            let already_visited =
                unsafe { board.get_unchecked(next_pos.x).get_unchecked(next_pos.y) };
            if !already_visited {
                stack.push((*next_pos, false));
            }
        }
    }
}

pub fn solve(dimensions: Pos, init_pos: Pos, sender: Sender<Vec<Pos>>) {
    let Pos {
        x: width,
        y: height,
    } = dimensions;

    let mut jump_acc = vec![];
    add_jumps(dimensions, init_pos, &mut jump_acc);

    let mut board = vec![vec![false; height]; width];
    board[init_pos.x][init_pos.y] = true;
    for jump in jump_acc {
        let thread_sender = sender.clone();
        let board = board.clone();
        let stack = vec![(init_pos, true), (jump, false)];
        std::thread::spawn(move || {
            solve_branch(dimensions, thread_sender, board, stack, 1, width * height);
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{self, Receiver, Sender};

    use super::{add_jumps, solve, Pos};

    #[test]
    fn jumps() {
        let mut acc = vec![];
        add_jumps(Pos { x: 4, y: 4 }, Pos { x: 0, y: 1 }, &mut acc);
        assert_eq!(
            acc,
            vec![Pos { x: 2, y: 2 }, Pos { x: 2, y: 0 }, Pos { x: 1, y: 3 }],
        );
    }

    #[test]
    fn boards() {
        {
            let (tx, rx): (Sender<Vec<Pos>>, Receiver<Vec<Pos>>) = mpsc::channel();
            solve(Pos { x: 3, y: 3 }, Pos { x: 0, y: 0 }, tx);
            let res = rx.recv().into_iter().count();
            assert_eq!(res, 0);
        }

        {
            let (tx, rx): (Sender<Vec<Pos>>, Receiver<Vec<Pos>>) = mpsc::channel();
            for i in 0..5 {
                let thread_tx = tx.clone();
                std::thread::spawn(move || {
                    for j in 0..5 {
                        solve(Pos { x: 5, y: 5 }, Pos { x: i, y: j }, thread_tx.clone());
                    }
                });
            }
            drop(tx);

            let mut num_solutions = 0;
            while let Ok(_) = rx.recv() {
                num_solutions += 1;
            }
            // https://en.m.wikipedia.org/wiki/Knight%27s_tour
            assert_eq!(num_solutions, 1728);
        }
    }
}
