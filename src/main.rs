use bytecount::num_chars;
use clap::Parser;
use std::{
    fs::read,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

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

    // Optional file to run operations on
    file_name: Option<String>,
}

enum InputSource {
    File(String),
    Stdin(String),
}

fn main() {
    let args = Args::parse();

    // Providing both file_name and stdin will just process the file.
    let input_source = match args.file_name {
        Some(file_name) => InputSource::File(file_name),
        None => {
            let mut content = String::new();
            /* If no input is provided, user is able to type in their own input
            then press Ctrl+D (sometimes twice). */
            let _ = io::stdin().read_to_string(&mut content);
            InputSource::Stdin(content)
        }
    };

    if !args.count && !args.lines && !args.words && !args.m {
        run_default(&input_source);
        return;
    } else {
        if args.count {
            if let Err(err) = count_file_bytes(&input_source) {
                eprintln!("Error: {:?}", err);
                return;
            }
        }
        if args.lines {
            if let Err(err) = count_file_lines(&input_source) {
                eprintln!("Error: {:?}", err);
                return;
            }
        }
        if args.words {
            if let Err(err) = count_file_words(&input_source) {
                eprintln!("Error: {:?}", err);
                return;
            }
        }
        if args.m {
            if let Err(err) = count_file_characters(&input_source) {
                eprintln!("Error: {:?}", err);
                return;
            }
        }
    }
}

fn count_file_bytes(source: &InputSource) -> io::Result<usize> {
    let byte_count = match source {
        InputSource::File(file_name) => {
            let file = read(file_name)?;
            file.len()
        }
        InputSource::Stdin(content) => content.as_bytes().len(),
    };
    println!("Bytes: {}", byte_count);
    Ok(byte_count)
}

fn count_file_lines(source: &InputSource) -> io::Result<usize> {
    let line_count = match source {
        InputSource::File(file_name) => {
            let reader = open_file(&file_name)?;
            reader.lines().count()
        }
        InputSource::Stdin(input) => input.lines().count(),
    };
    println!("Lines: {}", line_count);
    Ok(line_count)
}

fn count_file_words(source: &InputSource) -> io::Result<usize> {
    let word_count = match source {
        InputSource::File(file_name) => {
            let mut word_count_local = 0;
            let reader = open_file(&file_name)?;
            for line_result in reader.lines() {
                let line = line_result?;
                word_count_local += line.split_whitespace().count();
            }
            word_count_local
        }
        InputSource::Stdin(input) => {
            let mut word_count_local = 0;
            for line in input.lines() {
                word_count_local += line.split_whitespace().count()
            }
            word_count_local
        }
    };
    println!("Words: {}", word_count);
    Ok(word_count)
}

fn count_file_characters(source: &InputSource) -> io::Result<usize> {
    let character_count = match source {
        InputSource::File(file_name) => {
            let file = read(&file_name)?;
            num_chars(&file)
        }
        InputSource::Stdin(input) => num_chars(&input.as_bytes()),
    };
    println!("Characters: {}", character_count);
    Ok(character_count)
}

fn run_default(source: &InputSource) {
    if let Err(err) = count_file_bytes(source) {
        eprintln!("Error: {:?}", err);
        return;
    }
    if let Err(err) = count_file_lines(source) {
        eprintln!("Error: {:?}", err);
        return;
    }
    if let Err(err) = count_file_words(source) {
        eprintln!("Error: {:?}", err);
        return;
    }
}

fn open_file(file_name: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_file_bytes() -> io::Result<()> {
        let source = InputSource::File("test.txt".into());
        assert_eq!(count_file_bytes(&source)?, 342190); // including newline
        Ok(())
    }

    #[test]
    fn test_count_file_lines() -> io::Result<()> {
        let source = InputSource::File("test.txt".into());
        assert_eq!(count_file_lines(&source)?, 7145);
        Ok(())
    }

    #[test]
    fn test_count_file_words() -> io::Result<()> {
        let source = InputSource::File("test.txt".into());
        assert_eq!(count_file_words(&source)?, 58164);
        Ok(())
    }

    #[test]
    fn test_count_file_characters() -> io::Result<()> {
        let source = InputSource::File("test.txt".into());
        assert_eq!(count_file_characters(&source)?, 339292);
        Ok(())
    }
}
