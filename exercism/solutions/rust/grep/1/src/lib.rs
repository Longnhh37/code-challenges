use anyhow::Error;
use regex_lite::RegexBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Flags {
    show_line_numbers: bool,
    show_only_file_name: bool,
    ignore_case: bool,
    inverse: bool,
    full_line_match: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut parsed = Self {
            show_line_numbers: false,
            show_only_file_name: false,
            ignore_case: false,
            inverse: false,
            full_line_match: false,
        };

        for flag in flags {
            match *flag {
                "-n" => parsed.show_line_numbers = true,
                "-l" => parsed.show_only_file_name = true,
                "-i" => parsed.ignore_case = true,
                "-v" => parsed.inverse = true,
                "-x" => parsed.full_line_match = true,
                _ => unreachable!(),
            }
        }

        parsed
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = if flags.full_line_match {
        format!("^(?:{})$", pattern)
    } else {
        pattern.to_string()
    };

    let re = RegexBuilder::new(&pattern)
        .case_insensitive(flags.ignore_case)
        .build()
        .map_err(Error::new)?;

    let mut result: Vec<String> = Vec::new();
    let show_filename = files.len() > 1;

    'file: for file in files {
        let reader = BufReader::new(File::open(file)?);

        for (i, line) in reader.lines().enumerate() {
            let line = line?;

            if re.is_match(&line) ^ flags.inverse {
                if flags.show_only_file_name {
                    result.push(file.to_string());
                    continue 'file;
                }

                let mut out = String::new();

                if show_filename {
                    out.push_str(file);
                    out.push(':');
                }

                if flags.show_line_numbers {
                    out.push_str(&(i + 1).to_string());
                    out.push(':');
                }

                out.push_str(&line);
                result.push(out);
            }
        }
    }

    Ok(result)
}
