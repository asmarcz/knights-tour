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

pub fn solve(dimensions: Pos, init_pos: Pos) -> Vec<Vec<Pos>> {
    let mut solutions: Vec<Vec<Pos>> = vec![];
    let Pos {
        x: width,
        y: height,
    } = dimensions;
    let mut board = vec![vec![false; height]; width];

    let mut visited_sq_cnt = 0;
    let mut stack = vec![(init_pos, false)];

    let mut jump_acc = vec![];
    while !stack.is_empty() {
        let (curr_pos, should_close) = stack.pop().unwrap();
        let curr_square = &mut board[curr_pos.x][curr_pos.y];
        if should_close {
            *curr_square = false;
            visited_sq_cnt -= 1;
            continue;
        }
        *curr_square = true;
        visited_sq_cnt += 1;
        stack.push((curr_pos, true));

        if visited_sq_cnt == width * height {
            solutions.push(stack.iter().filter(|p| p.1).map(|p| p.0).collect());
        }

        jump_acc.truncate(0);
        add_jumps(dimensions, curr_pos, &mut jump_acc);
        for next_pos in &jump_acc {
            if !board[next_pos.x][next_pos.y] {
                stack.push((*next_pos, false));
            }
        }
    }

    solutions
}

#[cfg(test)]
mod tests {
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
        assert_eq!(
            solve(Pos { x: 3, y: 3 }, Pos { x: 0, y: 0 }),
            Vec::<Vec<Pos>>::new()
        );

        let mut num_solutions = 0;
        for i in 0..5 {
            for j in 0..5 {
                num_solutions += solve(Pos { x: 5, y: 5 }, Pos { x: i, y: j }).len();
            }
        }
        // https://en.m.wikipedia.org/wiki/Knight%27s_tour
        assert_eq!(num_solutions, 1728);
    }
}
