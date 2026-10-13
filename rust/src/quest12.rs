use std::collections::BTreeMap;

use ndarray::ArrayView2;

use crate::{
    Quest,
    QuestResult::{self, Number},
    util::input_to_grid_mut,
};

pub const PARTS: Quest = [part1, part2, part3];

fn shoot(
    grid: ArrayView2<u8>,
    y: usize,
    x: usize,
    power: usize,
) -> Option<[usize; 2]> {
    // dbg!(x);
    // dbg!(y);
    // dbg!(power);

    let trj = (0..power)
        .scan([y, x], |[y, x], _| {
            *x += 1;
            *y = y.wrapping_sub(1);

            Some([*y, *x])
        })
        .chain((0..power).scan(
            [y.wrapping_sub(power), x + power],
            |[y, x], _| {
                *x += 1;

                Some([*y, *x])
            },
        ));

    let mut x = x + 2 * power;
    let mut y = y.wrapping_sub(power);

    let trj = trj.chain((0..).map_while(|_| {
        x += 1;
        y = y.wrapping_add(1);

        (y != grid.dim().0).then_some([y, x])
    }));

    for [y, x] in trj {
        // println!("{:?}", [y, x]);
        if let (Some(&b'T'), Some(&b'.') | None) =
            (grid.get([y, x]), grid.get([y.wrapping_sub(1), x]))
        {
            return Some([y, x]);
        }
    }

    None
}

fn part1(mut input: String) -> QuestResult {
    input.reserve_exact(1);
    input.push('\n');
    let mut grid = input_to_grid_mut(unsafe { input.as_bytes_mut() });

    let mut towers = BTreeMap::new();

    for r in grid.rows().into_iter() {
        for (j, &x) in r.into_iter().enumerate() {
            if x == b'T' {
                if let Some(h) = towers.get_mut(&j) {
                    *h += 1;
                } else {
                    towers.insert(j, 1);
                }
            }
        }
    }

    let (h, _) = grid.dim();

    let base = [h - 2, 1];

    let mut ans = 0;

    while !towers.is_empty() {
        let (hit, power, height) = (1..)
            .flat_map(|power| (0..3).map(move |height| (power, height)))
            .find_map(|(power, height)| {
                shoot(grid.view(), base[0] - height, base[1], power)
                    .map(|hit| (hit, power, height))
            })
            .unwrap();

        ans += power * (height + 1);

        grid[hit] = b'.';
        let Some(v) = towers.get_mut(&hit[1]) else {
            panic!();
        };

        *v -= 1;

        if *v == 0 {
            towers.remove(&hit[1]);
        }
    }

    Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
