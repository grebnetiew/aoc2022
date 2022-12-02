#[aoc_generator(day2)]
pub fn char_space_char_per_line(input: &str) -> Vec<[char; 2]> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| [l.chars().next().unwrap(), l.chars().last().unwrap()])
        .collect()
}

/// Returns the total score.
#[aoc(day2, part1)]
pub fn part1(input: &[[char; 2]]) -> u32 {
    input
        .iter()
        // Returns your score. ABC (them) and XYZ (you) are rock, paper and
        // scissors scoring 1-2-3 for your choice; winning-drawing-losing is 6-3-0.
        .map(|pair| match pair {
            ['A', 'X'] => 1 + 3,
            ['A', 'Y'] => 2 + 6,
            ['A', 'Z'] => 3 + 0,
            ['B', 'X'] => 1 + 0,
            ['B', 'Y'] => 2 + 3,
            ['B', 'Z'] => 3 + 6,
            ['C', 'X'] => 1 + 6,
            ['C', 'Y'] => 2 + 0,
            ['C', 'Z'] => 3 + 3,
            _ => panic!("Incorrect input {:?}", pair),
        })
        .sum()
}
/// Returns the total score.
#[aoc(day2, part2)]
pub fn part2(input: &[[char; 2]]) -> u32 {
    input
        .iter()
        // Returns your score. ABC (them) and XYZ (you) are rock, paper and
        // scissors scoring 1-2-3 for your choice; winning-drawing-losing is 6-3-0.
        .map(|pair| match pair {
            ['A', 'X'] => 3 + 0,
            ['A', 'Y'] => 1 + 3,
            ['A', 'Z'] => 2 + 6,
            ['B', 'X'] => 1 + 0,
            ['B', 'Y'] => 2 + 3,
            ['B', 'Z'] => 3 + 6,
            ['C', 'X'] => 2 + 0,
            ['C', 'Y'] => 3 + 3,
            ['C', 'Z'] => 1 + 6,
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
        assert_eq!(part1(&char_space_char_per_line(TEST_INPUT)), 15);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&char_space_char_per_line(TEST_INPUT)), 12);
    }
}
