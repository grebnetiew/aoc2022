pub fn one_u32_per_line(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc_generator(day1)]
pub fn one_u32_per_line_grouped_by_empty_lines(
    input: &str,
) -> Result<Vec<Vec<u32>>, std::num::ParseIntError> {
    input.split("\n\n").map(one_u32_per_line).collect()
}

/// Returns the sum of the group with the largest sum.
#[aoc(day1, part1)]
pub fn part1(input: &[Vec<u32>]) -> Option<u32> {
    input.iter().map(|v| v.iter().sum()).max()
}

/// Returns the sum of the three groups with the largest sum.
#[aoc(day1, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let mut sums: Vec<u32> = input.iter().map(|v| v.iter().sum()).collect();
    sums.sort();
    sums.reverse();
    sums[0] + sums[1] + sums[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn sample1() {
        assert_eq!(
            part1(&one_u32_per_line_grouped_by_empty_lines(TEST_INPUT).unwrap()),
            Some(24000)
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            part2(&one_u32_per_line_grouped_by_empty_lines(TEST_INPUT).unwrap()),
            45000
        );
    }
}
