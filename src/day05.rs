#[derive(Debug)]
pub struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[aoc_generator(day5)]
pub fn crates_of_hanoi(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut input = input.split("\n\n");

    // First parse the crate lines
    let part1 = input.next().unwrap();
    // Start with the 1 2 3 line to see how many stacks we need
    let mut part1_lines = part1.lines().rev();
    let num_stacks = (part1_lines.next().unwrap().len() + 1) / 4;
    let mut stacks = vec![Vec::new(); num_stacks];
    // Parse the remaining lines
    for line in part1_lines {
        for (idx, stack) in stacks.iter_mut().enumerate() {
            if let Some(letter) = line.as_bytes().get(idx * 4 + 1) {
                if *letter != b' ' {
                    stack.push(*letter as char);
                }
            }
        }
    }

    // Now parse the moves
    let moves = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            // "move x from y to z"
            let mut numbers = line
                .split_ascii_whitespace()
                .skip(1)
                .step_by(2)
                .map(|s| s.parse().unwrap());
            // Correct the offset caused by numbering stacks from 1
            Move {
                amount: numbers.next().unwrap(),
                from: numbers.next().unwrap() - 1,
                to: numbers.next().unwrap() - 1,
            }
        })
        .collect();

    (stacks, moves)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let mut stacks = input.0.clone();
    for mv in &input.1 {
        for _ in 0..mv.amount {
            let letter = stacks[mv.from]
                .pop()
                .expect("stack empty during processing");
            stacks[mv.to].push(letter);
        }
    }

    stacks
        .into_iter()
        .map(|mut stack| stack.pop().expect("stack empty at the end"))
        .collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let mut stacks = input.0.clone();
    for mv in &input.1 {
        let start_at = stacks[mv.from].len() - mv.amount;
        let mut moving: Vec<_> = stacks[mv.from].drain(start_at..).collect();
        stacks[mv.to].append(&mut moving);
    }

    stacks
        .into_iter()
        .map(|mut stack| stack.pop().expect("stack empty at the end"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n 1   2   3 \n\
        \n\
        move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&crates_of_hanoi(TEST_INPUT)), String::from("CMZ"));
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&crates_of_hanoi(TEST_INPUT)), String::from("MCD"));
    }
}
