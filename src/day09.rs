use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl TryFrom<u8> for Direction {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'U' => Ok(Direction::North),
            b'D' => Ok(Direction::South),
            b'L' => Ok(Direction::West),
            b'R' => Ok(Direction::East),
            _ => Err(value),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    amount: usize,
}

#[aoc_generator(day9)]
pub fn move_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    input
        .lines()
        .map(|l| {
            Ok(Instruction {
                direction: Direction::try_from(l.as_bytes()[0])
                    .map_err(|b| format!("Can not convert {b} to direction"))?,
                amount: l[2..]
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?,
            })
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    // Yes, we can reduce part 1 and part 2 to the same function called with
    // the number of knots. But you might as well get to look at my initial
    // 'not so general' hack :)
    let mut visited = HashSet::<(i64, i64)>::new();
    let (mut xh, mut yh, mut xt, mut yt) = (0i64, 0i64, 0i64, 0i64);

    for ins in input {
        for _step in 0..ins.amount {
            match ins.direction {
                Direction::North => {
                    yh -= 1;
                    if yt - yh > 1 {
                        yt -= 1;
                        xt += (xh - xt).signum();
                    }
                }
                Direction::South => {
                    yh += 1;
                    if yh - yt > 1 {
                        yt += 1;
                        xt += (xh - xt).signum();
                    }
                }
                Direction::West => {
                    xh -= 1;
                    if xt - xh > 1 {
                        xt -= 1;
                        yt += (yh - yt).signum();
                    }
                }
                Direction::East => {
                    xh += 1;
                    if xh - xt > 1 {
                        xt += 1;
                        yt += (yh - yt).signum();
                    }
                }
            }
            visited.insert((xt, yt));
        }
    }

    visited.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Instruction]) -> usize {
    let mut visited = HashSet::<(i64, i64)>::new();
    let (mut x, mut y) = (vec![0i64; 10], vec![0i64; 10]);

    for ins in input {
        for _step in 0..ins.amount {
            match ins.direction {
                Direction::North => {
                    y[0] -= 1;
                }
                Direction::South => {
                    y[0] += 1;
                }
                Direction::West => {
                    x[0] -= 1;
                }
                Direction::East => {
                    x[0] += 1;
                }
            }
            follow(&mut x, &mut y, 1);
            visited.insert((x[9], y[9]));
        }
    }

    visited.len()
}

/// Updates the current position of `knot` after its predecessor moved,
/// and then recurses for the next knot
fn follow(x: &mut [i64], y: &mut [i64], knot: usize) {
    if knot >= x.len() {
        return;
    }

    // YYXYY  If knot is at the . positions relative to Head, nothing
    // Y...Y  happens. If it is at the X positions, it moves cardinally,
    // X.H.X  and recurses. If it is at the Y positions, it moves
    // Y...Y  diagonally, and recurses.
    // YYXYY  Signum nicely reduces the XY cases to one :)
    if (x[knot] - x[knot - 1]).abs() <= 1 && (y[knot] - y[knot - 1]).abs() <= 1 {
        return;
    }

    x[knot] += (x[knot - 1] - x[knot]).signum();
    y[knot] += (y[knot - 1] - y[knot]).signum();
    follow(x, y, knot + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        R 4\n\
        U 4\n\
        L 3\n\
        D 1\n\
        R 4\n\
        D 1\n\
        L 5\n\
        R 2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&move_instructions(TEST_INPUT).unwrap()), 13);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&move_instructions(TEST_INPUT).unwrap()), 1);
    }
}
