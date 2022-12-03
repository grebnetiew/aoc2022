use std::collections::HashSet;

/// Each line is split in halves, and the bytes in each half are collected as
/// HashSets
#[aoc_generator(day3)]
pub fn lines_split_to_hashsets(input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|l| {
            (
                l[..l.len() / 2].iter().cloned().collect(),
                l[l.len() / 2..].iter().cloned().collect(),
            )
        })
        .collect()
}

/// Computes 'priority': a = 1 .. z = 26, A = 27 .. Z = 52
pub fn priority(letter: u8) -> u32 {
    match letter {
        b'a'..=b'z' => (letter - b'a' + 1) as u32,
        b'A'..=b'Z' => (letter - b'A' + 27) as u32,
        _ => 0,
    }
}

/// Finds the common letter in each pair of HashSets, and then sums the
/// priorities of those common letters.
#[aoc(day3, part1)]
pub fn part1(input: &[(HashSet<u8>, HashSet<u8>)]) -> u32 {
    input
        .iter()
        .map(|pair| priority(*pair.0.intersection(&pair.1).next().unwrap()))
        .sum()
}

/// Finds the intersection (exactly one byte) of three HashSets of bytes
pub fn intersect3(three_hashsets: &[HashSet<u8>]) -> u8 {
    let intersect2: HashSet<u8> = three_hashsets[0]
        .intersection(&three_hashsets[1])
        .cloned()
        .collect();
    intersect2
        .intersection(&three_hashsets[2])
        .cloned()
        .next()
        .unwrap()
}

/// Merges the pairs of HashSets. Then for each set of three merged HashSets,
/// finds the letter that's common to all three sets, and sums the priorities
/// of those common letters.
#[aoc(day3, part2)]
pub fn part2(input: &[(HashSet<u8>, HashSet<u8>)]) -> u32 {
    let merged: Vec<HashSet<u8>> = input
        .iter()
        .map(|pair| pair.0.union(&pair.1).cloned().collect::<HashSet<u8>>())
        .collect();
    merged.chunks(3).map(intersect3).map(priority).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
    	vJrwpWtwJgWrhcsFMMfFFhFp\n\
		jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
		PmmdzqPrVvPwwTWBwg\n\
		wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
		ttgJtRGJQctTZtZT\n\
		CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn sample1() {
        assert_eq!(part1(&lines_split_to_hashsets(TEST_INPUT)), 157);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&lines_split_to_hashsets(TEST_INPUT)), 70);
    }
}
