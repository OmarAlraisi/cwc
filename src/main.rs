mod options;
mod stats;

use options::Options;
use stats::Stats;

use std::{fs, process::exit};

fn main() {
    let options = match Options::parse_options() {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    let mut status_code = 0;
    let mut stats: Vec<Stats> = vec![];

    for file_name in &options.file_names {
        match get_stats(file_name, &options) {
            Ok(stat) => {
                stat.display(&options, file_name);
                stats.push(stat);
            }
            Err(err) => {
                eprintln!("{}", err.0);
                status_code = 1;
            }
        }
    }

    if options.file_names.len() != 1 {
        let mut total = Stats::new(0, 0, 0, 0, 0);
        for stat in stats {
            total.lines += stat.lines;
            total.bytes += stat.bytes;
            total.words += stat.words;
            total.characters += stat.characters;

            if stat.len_of_most_bytes > total.len_of_most_bytes {
                total.len_of_most_bytes = stat.len_of_most_bytes;
            }
        }

        total.display(&options, &String::from("total"));
    }

    exit(status_code);
}

struct FileNotFoundError(String);

fn get_stats(file_name: &String, options: &Options) -> Result<Stats, FileNotFoundError> {
    match fs::read_to_string(&file_name) {
        Ok(content) => {
            let mut len_of_most_bytes = 0;
            let lines = content.lines().collect::<Vec<&str>>();
            let characters = if options.characters {
                content.chars().count()
            } else {
                0
            };
            let bytes = if options.bytes {
                content.bytes().len()
            } else {
                0
            };

            if options.most_bytes {
                for line in &lines {
                    let byte_len = line.bytes().len();
                    if byte_len > len_of_most_bytes {
                        len_of_most_bytes = byte_len;
                    }
                }
            }

            // this is really slow
            let words = if options.words {
                content.chars().filter(|c| !c.is_whitespace()).count()
            } else {
                0
            };

            Ok(Stats::new(
                lines.len(),
                bytes,
                words,
                characters,
                len_of_most_bytes,
            ))
        }
        Err(_) => Err(FileNotFoundError(format!(
            "wcw: {}: open: No such file or directory",
            file_name
        ))),
    }
}
