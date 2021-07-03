use grid::Grid;
use std::cmp::max;
// For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    io::BufReader::new(file).lines().collect()
}

#[allow(clippy::ptr_arg)]
fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    let (seq_1_len, seq_2_len) = (seq1.len(), seq2.len());
    let mut grid = Grid::new(seq_1_len + 1, seq_2_len + 1);
    (0..seq_1_len).for_each(|i| {
        (0..seq_2_len).for_each(|j| {
            if seq1[i] == seq2[j] {
                grid.set(i + 1, j + 1, grid.get(i, j).unwrap() + 1).unwrap();
            } else {
                grid.set(
                    i + 1,
                    j + 1,
                    max(grid.get(i + 1, j).unwrap(), grid.get(i, j + 1).unwrap()),
                )
                .unwrap();
            }
        });
    });
    grid
}

#[allow(clippy::ptr_arg)]
fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
        print_diff(lcs_table, lines1, lines2, i - 1, j - 1);
        println!("  {}", lines1[i - 1])
    } else if j > 0
        && (i == 0 || lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i - 1, j).unwrap())
    {
        print_diff(lcs_table, lines1, lines2, i, j - 1);
        println!("> {}", lines2[j - 1])
    } else if i > 0
        && (j == 0 || lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i - 1, j).unwrap())
    {
        print_diff(lcs_table, lines1, lines2, i - 1, j);
        println!("< {}", lines1[i - 1])
    } else {
        println!()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let file_one = read_file_lines(&filename1).expect("The file is invalid or does not exist");
    let file_two = read_file_lines(&filename2).expect("The file is invalid or does not exist");

    let result = lcs(&file_one, &file_two);
    // println!("{:?}", file_one);
    // println!("{:?}", file_two);
    // println!("{}", result);
    print_diff(
        &result,
        &file_one,
        &file_two,
        file_one.len(),
        file_two.len(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:\n{}", expected);
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:\n{}", result);
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
