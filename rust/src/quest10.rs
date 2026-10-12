use ndarray::{ArrayView2, s};

use crate::{
    Quest,
    QuestResult::{self, Text},
    util::input_to_grid,
};

pub const PARTS: Quest = [part1, part2, part3];

fn solve_string(grid: ArrayView2<u8>) -> [u8; 16] {
    let mut vertical = [0u32; 4];
    let mut horizontal = [0u32; 4];

    for (i, c1) in (2..6).enumerate() {
        for c2 in (0..2).chain(6..8) {
            let c = grid[[c1, c2]];
            horizontal[i] |= 1 << (c - b'A');

            let c = grid[[c2, c1]];
            vertical[i] |= 1 << (c - b'A');
        }
    }

    let mut ans = [0; _];

    for ((h, v), c) in horizontal
        .into_iter()
        .flat_map(|h| vertical.into_iter().map(move |v| (h, v)))
        .zip(&mut ans)
    {
        *c = (h & v).trailing_zeros() as u8;
    }

    ans
}

fn part1(mut input: String) -> QuestResult {
    input.reserve_exact(1);
    input.push('\n');

    let grid = input_to_grid(input.as_bytes());

    let values = solve_string(grid);

    let ans = values.into_iter().map(|c| (c + b'A') as char).collect();

    Text(ans)
}

fn part2(mut input: String) -> QuestResult {
    let linelength = input.lines().next().unwrap().len();

    input.reserve_exact(3);
    input.push('\n');
    input.push('\n');
    input.push('\n');

    let file_height = input.len() / ((linelength + 1) * 8 + 1);

    dbg!(file_height);

    let grid =
        ArrayView2::from_shape([file_height, linelength + 1], input.as_bytes())
            .unwrap();

    println!("{:?}", grid);

    let super_w = (linelength + 1) / 9;

    dbg!(super_w);

    let grid = grid.into_shape_with_order([]);

    // let h = grid.dim().0 / 9;
    // let w = grid.dim().1 / 9;

    // let grid = grid.into_shape_with_order([9, 9, h, w]).unwrap();

    // println!("{grid}");

    todo!()
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
