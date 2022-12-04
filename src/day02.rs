#[aoc_generator(day2)]
pub fn u8_space_u8_per_line(input: &str) -> Vec<[u8; 2]> {
    input
        // Working with bytes is ergonomic and faster (char boundaries are hard
        // because it might be super complex UTF-8 - but it's not, this time)
        .as_bytes()
        // Each line is four bytes: "A X\n"
        .chunks(4)
        .map(|chunk| [chunk[0], chunk[2]])
        .collect()
}

/// Returns the total score.
#[aoc(day2, part1)]
pub fn part1(input: &[[u8; 2]]) -> u32 {
    #[allow(clippy::identity_op)]
    input
        .iter()
        // Returns your score. ABC (them) and XYZ (you) are rock, paper and
        // scissors scoring 1-2-3 for your choice; winning-drawing-losing is 6-3-0.
        .map(|pair| match pair {
            [b'A', b'X'] => 1 + 3,
            [b'A', b'Y'] => 2 + 6,
            [b'A', b'Z'] => 3 + 0,
            [b'B', b'X'] => 1 + 0,
            [b'B', b'Y'] => 2 + 3,
            [b'B', b'Z'] => 3 + 6,
            [b'C', b'X'] => 1 + 6,
            [b'C', b'Y'] => 2 + 0,
            [b'C', b'Z'] => 3 + 3,
            _ => panic!("Incorrect input {:?}", pair),
        })
        .sum()
}

/// Returns the total score.
#[aoc(day2, part2)]
pub fn part2(input: &[[u8; 2]]) -> u32 {
    #[allow(clippy::identity_op)]
    input
        .iter()
        // Returns your score. ABC (them) is rock, paper and scissors, ZYX
        // (winning-drawing-losing) is 6-3-0, and the choice you have to make
        // to get that outcome gets you 1-2-3 points for rock-paper-scissors.
        .map(|pair| match pair {
            [b'A', b'X'] => 3 + 0,
            [b'A', b'Y'] => 1 + 3,
            [b'A', b'Z'] => 2 + 6,
            [b'B', b'X'] => 1 + 0,
            [b'B', b'Y'] => 2 + 3,
            [b'B', b'Z'] => 3 + 6,
            [b'C', b'X'] => 2 + 0,
            [b'C', b'Y'] => 3 + 3,
            [b'C', b'Z'] => 1 + 6,
            _ => panic!("Incorrect input {:?}", pair),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn sample1() {
        assert_eq!(part1(&u8_space_u8_per_line(TEST_INPUT)), 15);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&u8_space_u8_per_line(TEST_INPUT)), 12);
    }
}
