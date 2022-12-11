#[aoc_generator(day8)]
pub fn digit_matrix(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| (b - b'0') as i8).collect())
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(trees: &[Vec<i8>]) -> usize {
    let mut visible = vec![vec![false; trees[0].len()]; trees.len()];

    // Look on my for loops, ye mighty, and despair
    // Approach each line and column from both directions, marking trees we see
    for row in 0..trees.len() {
        let mut last_height = -1;
        for col in 0..trees[0].len() {
            if trees[row][col] > last_height {
                last_height = trees[row][col];
                visible[row][col] = true;
            }
            if last_height == 9 {
                break;
            }
        }

        let mut last_height = -1;
        for col in (0..trees[0].len()).rev() {
            if trees[row][col] > last_height {
                last_height = trees[row][col];
                visible[row][col] = true;
            }
            if last_height == 9 {
                break;
            }
        }
    }

    for col in 0..trees[0].len() {
        let mut last_height = -1;
        for row in 0..trees.len() {
            if trees[row][col] > last_height {
                last_height = trees[row][col];
                visible[row][col] = true;
            }
            if last_height == 9 {
                break;
            }
        }

        let mut last_height = -1;
        for row in (0..trees.len()).rev() {
            if trees[row][col] > last_height {
                last_height = trees[row][col];
                visible[row][col] = true;
            }
            if last_height == 9 {
                break;
            }
        }
    }

    visible
        .iter()
        .map(|line| line.iter().filter(|t| **t).count())
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(trees: &[Vec<i8>]) -> u64 {
    let mut max_score = 0;
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            max_score = std::cmp::max(max_score, find_score(trees, row, col));
        }
    }

    max_score
}

fn find_score(trees: &[Vec<i8>], row: usize, col: usize) -> u64 {
    let current_tree = trees[row][col];

    // Look in all four directions
    let mut score_u = 0;
    for r in (0..row).rev() {
        score_u += 1;
        if trees[r][col] >= current_tree {
            break;
        }
    }

    let mut score_d = 0;
    #[allow(clippy::needless_range_loop)]
    for r in row + 1..trees.len() {
        score_d += 1;
        if trees[r][col] >= current_tree {
            break;
        }
    }

    let mut score_l = 0;
    for c in (0..col).rev() {
        score_l += 1;
        if trees[row][c] >= current_tree {
            break;
        }
    }

    let mut score_r = 0;
    for c in col + 1..trees[0].len() {
        score_r += 1;
        if trees[row][c] >= current_tree {
            break;
        }
    }

    score_u * score_l * score_d * score_r
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
	30373\n\
	25512\n\
	65332\n\
	33549\n\
	35390";

    #[test]
    fn sample1() {
        assert_eq!(part1(&digit_matrix(TEST_INPUT)), 21);
    }

    #[test]
    fn score() {
        assert_eq!(find_score(&digit_matrix(TEST_INPUT), 3, 2), 8);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&digit_matrix(TEST_INPUT)), 8);
    }
}
