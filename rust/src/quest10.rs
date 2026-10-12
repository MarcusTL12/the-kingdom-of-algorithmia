use crate::{
    Quest,
    QuestResult::{self, Text},
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut lines = input.lines();

    let mut vertical = [0u32; 4];
    let mut horizontal = [0u32; 4];

    for _ in 0..2 {
        let l = lines.next().unwrap();

        for (i, c) in l.chars().skip(2).take(4).enumerate() {
            vertical[i] |= 1 << ((c as u8) - b'A');
        }
    }

    for h in &mut horizontal {
        let l = lines.next().unwrap();
        for c in l.chars().filter(|&c| c != '.') {
            *h |= 1 << ((c as u8) - b'A');
        }
    }

    for l in lines {
        for (i, c) in l.chars().skip(2).take(4).enumerate() {
            vertical[i] |= 1 << ((c as u8) - b'A');
        }
    }

    let mut ans = String::new();
    ans.reserve_exact(16);

    for h in horizontal {
        for v in vertical {
            let c = (h & v).trailing_zeros() as u8 + b'A';

            ans.push(c as char);
        }
    }

    Text(ans)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
