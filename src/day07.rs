use std::collections::HashMap;

/// Parses the listing and returns a list of full filenames and their sizes
#[aoc_generator(day7)]
pub fn directory_listing_parser(
    input: &str,
) -> Result<HashMap<String, usize>, std::num::ParseIntError> {
    let mut pwd = String::new(); // "" is the root directory
    let mut files = HashMap::new();

    for line in input.lines() {
        match &line[0..4] {
            "$ cd" => match line.as_bytes()[5] {
                b'/' => pwd.clear(),
                b'.' => pwd.truncate(pwd.rfind('/').expect("cd.. too far!")),
                _ => pwd = pwd + "/" + &line[5..],
            },
            "$ ls" | "dir " => {}
            _ => {
                let mut components = line.split_ascii_whitespace();
                let size = components.next().unwrap().parse()?;
                let name = components.next().unwrap();
                files.insert(pwd.clone() + "/" + name, size);
            }
        }
    }

    Ok(files)
}

/// Take all files and add their sizes to all dirs they're part of
fn dir_sizes(files: &HashMap<String, usize>) -> HashMap<String, usize> {
    // Recursively add a file's size to all directories above
    fn process(dirs: &mut HashMap<String, usize>, file: &str, size: usize) {
        // Remove file (or dir) name from the full path
        let file = &file[..file.rfind('/').expect("recusion base case error")];
        // Add the size of the contained file to this path
        *dirs.entry(file.to_owned()).or_insert(0) += size;
        // Unless at root dir, recurse
        if !file.is_empty() {
            process(dirs, file, size);
        }
    }

    let mut dirs = HashMap::new();

    for (file, size) in files {
        process(&mut dirs, file, *size);
    }

    dirs
}

/// Given the list of file sizes, find the directories with size-on-disk
/// smaller than 100000, and sum the sizes
#[aoc(day7, part1)]
pub fn part1(files: &HashMap<String, usize>) -> usize {
    let dirs = dir_sizes(files);

    dirs.values().filter(|sz| **sz <= 100_000).sum()
}

/// Given the list of file sizes, find the smallest directory that must be
/// deleted so there's 30M free space, and return its size
#[aoc(day7, part2)]
pub fn part2(files: &HashMap<String, usize>) -> Option<usize> {
    let dirs = dir_sizes(files);

    let disk_usage = dirs[""];
    let disk_free = 70_000_000usize.checked_sub(disk_usage)?;
    let disk_needed = 30_000_000usize.checked_sub(disk_free)?;

    dirs.values()
        .filter(|sz| **sz >= disk_needed)
        .min()
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        $ cd /\n\
        $ ls\n\
        dir a\n\
        14848514 b.txt\n\
        8504156 c.dat\n\
        dir d\n\
        $ cd a\n\
        $ ls\n\
        dir e\n\
        29116 f\n\
        2557 g\n\
        62596 h.lst\n\
        $ cd e\n\
        $ ls\n\
        584 i\n\
        $ cd ..\n\
        $ cd ..\n\
        $ cd d\n\
        $ ls\n\
        4060174 j\n\
        8033020 d.log\n\
        5626152 d.ext\n\
        7214296 k";

    #[test]
    fn sample1() {
        assert_eq!(part1(&directory_listing_parser(TEST_INPUT).unwrap()), 95437);
    }
    #[test]
    fn sample2() {
        assert_eq!(
            part2(&directory_listing_parser(TEST_INPUT).unwrap()),
            Some(24933642)
        );
    }
}
