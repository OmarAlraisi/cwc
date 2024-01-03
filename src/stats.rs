use crate::options::Options;
pub struct Stats {
    pub lines: usize,
    pub bytes: usize,
    pub words: usize,
    pub characters: usize,
    pub len_of_most_bytes: usize,
}
impl Stats {
    pub fn new(
        lines: usize,
        bytes: usize,
        words: usize,
        characters: usize,
        len_of_most_bytes: usize,
    ) -> Self {
        Stats {
            lines,
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
}
