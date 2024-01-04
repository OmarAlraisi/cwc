mod options;
mod stats;

use options::Options;
use stats::Stats;

use std::{fs, process::exit};

fn main() {
    let (options, file_names) = match Options::parse_options() {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    let mut status_code = 0;
    let mut stats: Vec<Stats> = vec![];

    for file_name in &file_names {
        match fs::read_to_string(file_name) {
            Ok(content) => {
                let stat = Stats::get_stats(content, &options);
                stat.display(&options, file_name);
                stats.push(stat);
            }
            Err(_) => {
                eprintln!("wcw: {}: open: No such file or directory", file_name);
                status_code = 1;
            }
        }
    }

    if file_names.len() > 1 {
        Stats::display_total_stats(stats, &options);
    }

    exit(status_code);
}
