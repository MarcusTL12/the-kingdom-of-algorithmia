use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(mut input: String) -> QuestResult {
    if input.as_bytes().last().unwrap() != &b'\n' {
        input.push('\n');
    }

    let data = input.as_bytes();

    let w = data
        .iter()
        .enumerate()
        .find(|(_, &x)| x == b'\n')
        .map(|(i, _)| i)
        .unwrap();

    let h = data.len() / w;

    todo!()
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
