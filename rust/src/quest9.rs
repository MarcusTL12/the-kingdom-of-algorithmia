use std::{cmp::Reverse, collections::BTreeSet};

use priority_queue::PriorityQueue;

use crate::{
    Quest,
    QuestResult::{self, Number},
};

pub const PARTS: Quest = [part1, part2, part3];

const STAPMS: [u32; 4] = [1, 3, 5, 10];
const MAXSTAMP: u32 = 10;

fn find_min_number_of_stamps(target: u32) -> u32 {
    let h = |x: u32| x / MAXSTAMP;

    let mut queue = PriorityQueue::new();
    queue.push(target, Reverse(h(target)));
    let mut seen = BTreeSet::new();

    while let Some((rest, Reverse(d))) = queue.pop() {
        let n = d - h(rest);

        if rest == 0 {
            return n;
        }

        seen.insert(rest);

        for stamp in STAPMS.into_iter().rev() {
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
        .map(find_min_number_of_stamps)
        .sum();

    Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
