use std::{cmp::Reverse, collections::BTreeSet};

use priority_queue::PriorityQueue;
use rayon::{iter::ParallelIterator, str::ParallelString};

use crate::{
    Quest,
    QuestResult::{self, Number},
};

pub const PARTS: Quest = [part1, part2, part3];

fn find_min_number_of_stamps(stamps: &[u32], target: u32) -> u32 {
    let maxstamp = stamps.iter().max().unwrap();
    let h = |x: u32| x / maxstamp;

    let mut queue = PriorityQueue::new();
    queue.push(target, Reverse(h(target)));
    let mut seen = BTreeSet::new();

    while let Some((rest, Reverse(d))) = queue.pop() {
        let n = d - h(rest);

        if rest == 0 {
            return n;
        }

        seen.insert(rest);

        for &stamp in stamps.iter().rev() {
            if stamp <= rest && !seen.contains(&(rest - stamp)) {
                queue.push_increase(
                    rest - stamp,
                    Reverse(n + 1 + h(rest - stamp)),
                );
            }
        }
    }

    panic!()
}

fn part1(input: String) -> QuestResult {
    let ans: u32 = input
        .split('\n')
        .map(|x| x.parse().unwrap())
        .map(|x| find_min_number_of_stamps(&[1, 3, 5, 10], x))
        .sum();

    Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    let ans: u32 = input
        .split('\n')
        .map(|x| x.parse().unwrap())
        .map(|x| {
            find_min_number_of_stamps(&[1, 3, 5, 10, 15, 16, 20, 24, 25, 30], x)
        })
        .sum();

    Number(ans as i64)
}

fn find_best_split(target: u32) -> u32 {
    const STAMPS: &[u32] = &[
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];

    let m = target / 2;

    ((m - 50 + target % 2)..=m)
        .map(|l| {
            let h = target - l;

            let nl = find_min_number_of_stamps(STAMPS, l);
            let nh = find_min_number_of_stamps(STAMPS, h);

            nl + nh
        })
        .min()
        .unwrap()
}

fn part3(input: String) -> QuestResult {
    let ans: u32 = input
        .par_split('\n')
        .map(|x| x.parse().unwrap())
        .map(find_best_split)
        .sum();

    Number(ans as i64)
}
