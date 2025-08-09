use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long = "bytes")]
    count_bytes: bool,

    #[arg(short = 'l', long = "lines")]
    count_lines: bool,

    #[arg(short = 'w', long = "words")]
    count_words: bool,

    #[arg(short = 'm', long = "characters")]
    count_characters: bool,

    #[arg(default_value = "-")]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run_logic(&args.file_path, &args) {
        eprintln!("Application error in file {}: {}", &args.file_path, e);
    }
}

fn run_logic<P: AsRef<Path>>(path: P, options: &Args) -> io::Result<()> {
    let path = path.as_ref();

    let content = if path.to_str() == Some("-") {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    } else {
        match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => return Err(e),
        }
    };
    
    let option_not_selected = !options.count_bytes && !options.count_characters && !options.count_lines && !options.count_words;

    let mut ouptut = String::new();

    if options.count_bytes || option_not_selected {
       ouptut.push_str(&byte_count(&content).to_string());
       ouptut.push_str(" ");
    }
    if options.count_lines || option_not_selected {
        ouptut.push_str(&line_count(&content).to_string());
        ouptut.push_str(" ");
    }
    if options.count_words || option_not_selected {
        ouptut.push_str(&word_count(&content).to_string());
        ouptut.push_str(" ");
    }
    if options.count_characters || option_not_selected {
        ouptut.push_str(&character_count(&content).to_string());
        ouptut.push_str(" ");
    }

    if !(path.to_str() == Some("-")) {
        ouptut.push_str(&options.file_path)
    }

    println!("{}", ouptut);

    Ok(())
}

fn byte_count(text: &str) -> usize {
    text.len()
}

fn line_count(text: &str) -> usize {
    text.lines().count()
}

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn character_count(text: &str) -> usize {
    text.chars().count()
}