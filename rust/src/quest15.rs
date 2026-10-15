use std::collections::VecDeque;

use crate::{
    Quest,
    QuestResult::{self, Number},
    util::input_to_grid_mut,
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut bytes = input.into_bytes();
    bytes.reserve_exact(1);
    bytes.push(b'\n');
    let mut grid = input_to_grid_mut(&mut bytes);

    let sx = grid
        .row(0)
        .indexed_iter()
        .find_map(|(x, &v)| (v == b'.').then_some(x))
        .unwrap();

    grid[[0, sx]] = b'#';

    let mut queue = VecDeque::new();
    queue.push_back(([0, sx], 0));

    const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    while let Some((pos, l)) = queue.pop_front() {
        for dir in DIRS {
            let Some(newpos) =
                pos[0].checked_add_signed(dir[0]).and_then(|np0| {
                    pos[1].checked_add_signed(dir[1]).map(|np1| [np0, np1])
                })
            else {
                continue;
            };

            let Some(v) = grid.get_mut(newpos) else {
                continue;
            };

            match *v {
                b'.' => {
                    *v = b'#';
                    queue.push_back((newpos, l + 1));
                }
                b'H' => return Number(2 * (l + 1)),
                _ => {}
            }
        }
    }

    todo!()
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
