use crate::{
    Quest,
    QuestResult::{self, Text},
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut lines = input.lines();

    let steps: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    lines.next().unwrap();

    let mut wheels = vec![Vec::new(); steps.len()];

    for mut l in lines {
        for wheel in wheels.iter_mut() {
            let Some((cat, rest)) = l.split_at_checked(3) else {
                break;
            };

            if let Some((_, r)) = rest.split_at_checked(1) {
                l = r;
            } else {
                l = "";
            }

            if cat != "   " {
                wheel.push(cat);
            }
        }
    }

    let mut positions = vec![0; steps.len()];

    for _ in 0..100 {
        for ((pos, step), wheel) in
            positions.iter_mut().zip(&steps).zip(&wheels)
        {
            let l = wheel.len() as u32;
            *pos = (*pos + step) % l;
        }
    }

    Text(
        wheels
            .into_iter()
            .zip(positions)
            .map(|(wheel, i)| wheel[i as usize])
            .intersperse(" ")
            .collect(),
    )
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
