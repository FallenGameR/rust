use text_colorizer::*;
use std::env;

#[derive(Debug)]
struct Arguments {
    old_regex: String,
    new_text: String,
    in_path: String,
    out_path: String,
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);
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