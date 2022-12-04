/// Error type that results from `two_ranges_comma_separated`, and
/// can communicate both invalid numbers (e.g. `3-4,5-hi`) and incorrect
/// amounts of numbers (e.g. `1-2,3`).
#[derive(Debug)]
pub enum ParseError {
    /// An invalid digit was encountered
    ParseIntError(std::num::ParseIntError),
    /// Too few or too many numbers were encountered
    AmountOfNumbersError(usize),
}

use ParseError::*;

/// Expects lines of the form `u32-u32,u32-u32` and returns the four
/// numbers as arrays.
#[aoc_generator(day4)]
pub fn two_ranges_comma_separated(input: &str) -> Result<Vec<[u32; 4]>, ParseError> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .flat_map(|s| s.split('-').map(|s| s.parse().map_err(ParseIntError)))
                .collect::<Result<Vec<u32>, _>>()?
                .try_into()
                .map_err(|v: Vec<_>| AmountOfNumbersError(v.len()))
        })
        .collect()
}

/// Takes two closed intervals [a-b], [c-d], and returns whether one
/// interval fully contains (is subset of) another
pub fn ranges_fully_contained(ranges: &&[u32; 4]) -> bool {
    // Ugly argument type makes it convenient to use in `filter` :)
    (ranges[0] >= ranges[2] && ranges[1] <= ranges[3])
        || (ranges[2] >= ranges[0] && ranges[3] <= ranges[1])
}

/// Counts the number of pairs of ranges where one is fully contained
/// in the other
#[aoc(day4, part1)]
pub fn part1(input: &[[u32; 4]]) -> usize {
    input.iter().filter(ranges_fully_contained).count()
}

/// Takes two closed intervals [a-b], [c-d], and whether they overlap
pub fn ranges_overlap(ranges: &&[u32; 4]) -> bool {
    let overlap_begin = Ord::max(ranges[0], ranges[2]);
    let overlap_end = Ord::min(ranges[1], ranges[3]);
    overlap_end >= overlap_begin
}

/// Counts the number of pairs of ranges with overlap
#[aoc(day4, part2)]
pub fn part2(input: &[[u32; 4]]) -> usize {
    input.iter().filter(ranges_overlap).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn sample1() {
        assert_eq!(part1(&two_ranges_comma_separated(TEST_INPUT).unwrap()), 2);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&two_ranges_comma_separated(TEST_INPUT).unwrap()), 4);
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseIntError(inner) => std::fmt::Display::fmt(inner, f),
            AmountOfNumbersError(n) => write!(f, "a line contained {n} numbers instead of 4"),
        }
    }
}

impl std::error::Error for ParseError {}
