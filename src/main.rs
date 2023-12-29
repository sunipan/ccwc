use clap::Parser;
use std::{
    fs::read,
    fs::File,
    io::{self, BufRead, BufReader},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Option to count bytes of file
    #[arg(short)]
    count: bool,

    // Option to count lines of file
    #[arg(short)]
    lines: bool,

    // Option to count words of file
    #[arg(short)]
    words: bool,

    // Option to count characters of file
    #[arg(short)]
    m: bool,

    // File to run operations on
    file_name: String,
}

fn main() {
    let args = Args::parse();

    if !args.count && !args.lines && !args.words && !args.m {
        run_default(&args.file_name);
        return;
    } else {
        if args.count {
            count_file_bytes(&args.file_name);
        }
        if args.lines {
            count_file_lines(&args.file_name);
        }
        if args.words {
            count_file_words(&args.file_name);
        }
        if args.m {
            count_file_characters(&args.file_name);
        }
    }
}

fn count_file_bytes(file_name: &str) {
    let file_result = read(file_name);
    let file = match file_result {
        Ok(file_vec) => file_vec,
        Err(error) => {
            eprintln!("Problem reading error: {:?}", error);
            return;
        }
    };
    println!("Bytes: {}", file.len());
}

fn count_file_lines(file_name: &str) {
    let mut line_count = 0;
    let reader = open_file(&file_name);
    if let Ok(reader) = reader {
        for line in reader.lines() {
            if let Ok(_line) = line {
                line_count += 1;
            }
        }
        println!("Lines: {}", line_count);
    } else if let Err(error) = reader {
        eprintln!("Problem reading error: {:?}", error);
    }
}

fn count_file_words(file_name: &str) {
    let mut word_count = 0;
    let reader = open_file(&file_name);
    if let Ok(reader) = reader {
        for line in reader.lines() {
            if let Ok(line) = line {
                word_count += line.split_whitespace().count();
            }
        }
        println!("Words: {}", word_count);
    } else if let Err(error) = reader {
        eprintln!("Problem reading error: {:?}", error);
    }
}

fn count_file_characters(file_name: &str) {
    let mut char_count = 0;
    let reader = open_file(&file_name);
    if let Ok(reader) = reader {
        for line in reader.lines() {
            if let Ok(line) = line {
                char_count += line.chars().count();
            }
        }
        println!("Characters: {}", char_count);
    } else if let Err(error) = reader {
        eprintln!("Problem reading error: {:?}", error);
    }
}

fn run_default(file_name: &str) {
    count_file_bytes(&file_name);
    count_file_lines(&file_name);
    count_file_words(&file_name);
}

fn open_file(file_name: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file))
}
