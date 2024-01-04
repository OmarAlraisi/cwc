use crate::options::Options;

pub struct Stats {
    pub lines: usize,
    pub bytes: usize,
    pub words: usize,
    pub characters: usize,
    pub len_of_most_bytes: usize,
}

impl Stats {
    pub fn get_stats(content: String, options: &Options) -> Stats {
        let lines = if options.lines || options.most_bytes {
            content.lines().collect::<Vec<&str>>()
        } else {
            vec![]
        };

        let bytes = if options.bytes {
            content.bytes().len()
        } else {
            0
        };

        let characters = if options.characters && !options.bytes {
            content.chars().count()
        } else {
            0
        };

        let words = if options.words {
            content.split_whitespace().count()
        } else {
            0
        };

        let len_of_most_bytes = if options.most_bytes {
            lines.iter().map(|line| line.bytes().len()).max().unwrap()
        } else {
            0
        };

        Stats {
            lines: lines.len(),
            bytes,
            words,
            characters,
            len_of_most_bytes,
        }
    }

    pub fn display(&self, options: &Options, file_name: &String) {
        let mut output = String::new();

        if options.lines {
            output.push_str(format!("{:>7} ", self.lines).as_str());
        }

        if options.words {
            output.push_str(format!("{:>7} ", self.words).as_str());
        }

        if options.bytes {
            output.push_str(format!("{:>7} ", self.bytes).as_str());
        } else if options.characters {
            output.push_str(format!("{:>7} ", self.characters).as_str());
        }

        if options.most_bytes {
            output.push_str(format!("{:>7} ", self.len_of_most_bytes).as_str());
        }

        println!(" {}{}", output, file_name);
    }

    pub fn display_total_stats(stats: Vec<Stats>, options: &Options) {
        let mut total = Stats {
            lines: 0,
            bytes: 0,
            words: 0,
            characters: 0,
            len_of_most_bytes: 0,
        };

        for stat in stats {
            total.lines += stat.lines;
            total.bytes += stat.bytes;
            total.words += stat.words;
            total.characters += stat.characters;

            if stat.len_of_most_bytes > total.len_of_most_bytes {
                total.len_of_most_bytes = stat.len_of_most_bytes;
            }
        }

        total.display(options, &String::from("total"));
    }
}
