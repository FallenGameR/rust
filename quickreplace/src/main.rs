use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    old_regex: String,
    new_text: String,
    in_path: String,
    out_path: String,
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.in_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), args.in_path, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.old_regex, &args.new_text, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.out_path, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), args.in_path, e);
            std::process::exit(1);
        }
    };
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn print_usage() {
    eprintln!("{} - change occurences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <old_regex> <new_text> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        old_regex: args[0].clone(),
        new_text: args[1].clone(),
        in_path: args[2].clone(),
        out_path: args[3].clone()
    }
}
