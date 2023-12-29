use clap::Parser;

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
}

fn count_file_bytes() {}

fn count_file_lines() {}

fn count_file_words() {}

fn count_file_characters() {}
