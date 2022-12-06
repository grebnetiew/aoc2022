/// Returns whether all bytes are different
fn unique(slice: &[u8]) -> bool {
    if slice.len() == 1 {
        return true;
    }
    for x in &slice[1..] {
        if *x == slice[0] {
            return false;
        }
    }
    unique(&slice[1..])
}

/// Returns the first sequence index of `length` unique bytes
fn find_unique_sequence(input: &str, length: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(length)
        .enumerate()
        .find(|(_, win)| unique(win))
        .map(|(idx, _)| idx + length)
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> Option<usize> {
    find_unique_sequence(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> Option<usize> {
    find_unique_sequence(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    #[test]
    fn sample1() {
        assert_eq!(part1(TEST_INPUT), Some(5));
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TEST_INPUT), Some(23));
    }
}
