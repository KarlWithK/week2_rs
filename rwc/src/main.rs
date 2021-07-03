use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

// struct Stats {
//     name: String,
//     lines: usize,
//     words: usize,
//     characters: usize,
// }

fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    io::BufReader::new(file).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let print_total;
    match args.len() {
        0 | 1 => {
            println!("Too few arguments.");
            process::exit(1);
        }
        2 => print_total = false,
        _ => print_total = true,
    }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_characters = 0;
    for file in args.into_iter().skip(1) {
        let read_lines = read_file_lines(&file).expect("File should exist or be valid");
        let line_num = read_lines.len();
        let mut words = 0;
        let mut characters = 0;
        read_lines.into_iter().for_each(|line| {
            words += line.split_whitespace().count();
            characters += line.chars().count();
        });
        if print_total {
            total_lines += line_num;
            total_words += words;
            total_characters += characters;
        }
        println!("\t{}\t{}\t{}\t{}", line_num, words, characters, file)
    }

    if print_total {
        println!(
            "\t{}\t{}\t{}\ttotal",
            total_lines, total_words, total_characters
        )
    }
}
