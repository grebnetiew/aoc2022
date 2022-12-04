/// Expects lines of the form `u32-u32,u32-u32` and returns the four
/// numbers as arrays.
#[aoc_generator(day4)]
pub fn two_ranges_comma_separated(input: &str) -> Result<Vec<[u32; 4]>, std::num::ParseIntError> {
    input
        .lines()
        .map(|l| {
            Ok(l.split(',')
                .map(|s| s.split('-').map(str::parse).filter_map(Result::ok))
                .flatten()
                .collect::<Vec<u32>>()
                .try_into()
                // Would be better to return an error here too but
                // too much work ;)
                .expect("not all lines contain four numbers"))
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
