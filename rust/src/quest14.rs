use std::{collections::HashSet, iter::repeat_n};

use crate::{
    Quest,
    QuestResult::{self, Number},
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let (_, ans) = input
        .split(',')
        .filter_map(|r| match &r[0..1] {
            "U" => Some(r[1..].parse::<i64>().unwrap()),
            "D" => Some(-r[1..].parse::<i64>().unwrap()),
            _ => None,
        })
        .fold((0, 0), |(y, maxy), dy| (y + dy, maxy.max(y + dy)));

    Number(ans)
}

fn part2(input: String) -> QuestResult {
    let tiles: HashSet<_> = input
        .lines()
        .flat_map(|l| {
            l.split(',')
                .flat_map(|r| {
                    repeat_n(
                        match &r[0..1] {
                            "R" => [1, 0, 0],
                            "L" => [-1, 0, 0],
                            "U" => [0, 1, 0],
                            "D" => [0, -1, 0],
                            "F" => [0, 0, 1],
                            "B" => [0, 0, -1],
                            _ => panic!(),
                        },
                        r[1..].parse().unwrap(),
                    )
                })
                .scan([0; 3], |pos, dp| {
                    for (q, dq) in pos.iter_mut().zip(&dp) {
                        *q += dq;
                    }

                    Some(*pos)
                })
        })
        .collect();

    Number(tiles.len() as i64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
