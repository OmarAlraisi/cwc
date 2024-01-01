mod options;

use options::Options;
use std::{fs, process::exit};

fn main() {
    let options = match Options::parse_options() {
        Ok(options) => options,
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    };

    let files = options.file_names.clone();

    let mut status_code = 0;
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_characters = 0;
    let mut total_bytes = 0;
    let mut total_len_of_most_bytes = 0;

    for file in files {
        match fs::read_to_string(&file) {
            Ok(content) => {
                let mut len_of_most_bytes = 0;
                let lines = content.lines().collect::<Vec<&str>>();
                let characters = content.chars().collect::<Vec<char>>().len();
                let bytes = content.bytes().len();

                for line in &lines {
                    let byte_len = line.bytes().len();
                    if byte_len > len_of_most_bytes {
                        len_of_most_bytes = byte_len;
                    }
                }

                // This is wrong
                let words = lines
                    .clone()
                    .into_iter()
                    .map(|line| line.split(' '))
                    .flatten()
                    .collect::<Vec<&str>>()
                    .len();

                println!(
                    "{}\nl: {}\tc: {}\tw: {}\tm: {}\tL: {} total",
                    file, lines.len(), bytes, words, characters, len_of_most_bytes
                );

                total_lines += lines.len();
                total_bytes += bytes;
                total_words += words;
                total_characters += characters;
                total_len_of_most_bytes = len_of_most_bytes;
            }
            Err(_) => {
                eprintln!("wcw: {}: open: No such file or directory", file);
                status_code = 1;
            }
        }
    }

    println!(
        "L: {}\nc: {}\nw: {}\nm: {}\nl: {} total",
        total_lines, total_bytes, total_words, total_characters, total_len_of_most_bytes
    );

    exit(status_code);
}
