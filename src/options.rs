use std::{env, fmt::Display};

pub struct ParseError(String);
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Options {
    pub file_names: Vec<String>,
    most_bytes: bool,
    bytes: bool,
    lines: bool,
    characters: bool,
    words: bool,
}

impl Options {
    fn new(no_args: bool) -> Self {
        Options {
            file_names: vec![],
            most_bytes: false,
            bytes: no_args,
            lines: no_args,
            characters: false,
            words: no_args,
        }
    }

    fn add_file(&mut self, file_name: String) {
        self.file_names.push(file_name);
    }

    fn decode_options(&mut self, arg: String) -> Option<ParseError> {
        let mut chars = arg.chars();
        chars.next();

        while let Some(c) = chars.next() {
            match c {
                'L' => self.most_bytes = true,
                'c' => self.bytes = true,
                'l' => self.lines = true,
                'm' => self.characters = true,
                'w' => self.words = true,
                _ => {
                    return Some(ParseError(format!(
                        "wcw: illegal option -- {}\nusage: wcw [-Lclmw] [file ...]",
                        c
                    )));
                }
            }
        }

        None
    }

    pub fn parse_options() -> Result<Options, ParseError> {
        let args: Vec<String> = env::args().skip(1).collect();
        match args.len() {
            0 => Err(ParseError(String::from(
                "Error: Usage: wcw [-Lclmw] [file_name]",
            ))),
            1 => {
                if args[0].starts_with('-') {
                    Err(ParseError(String::from("Usage: wcw [-LclMw] [file_name]")))
                } else {
                    let mut options = Options::new(true);
                    options.add_file(args[0].clone());
                    Ok(options)
                }
            }
            _ => {
                let mut idx = 0;
                let mut options = Options::new(false);
                while idx != args.len() && args[idx].starts_with('-') {
                    if let Some(err) = options.decode_options(args[idx].clone()) {
                        return Err(err);
                    };
                    idx += 1;
                }

                if idx == args.len() {
                    return Err(ParseError(String::from("No input provided")));
                }

                while idx != args.len() {
                    options.add_file(args[idx].clone());
                    idx += 1;
                }

                Ok(options)
            }
        }
    }
}
