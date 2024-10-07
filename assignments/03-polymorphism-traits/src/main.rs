use anyhow::Result;
use clap::Parser;
use colored::Color;
use std::path::PathBuf;

mod input;
mod output;
mod search;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ignore_case: bool,

    #[clap(short = 'v', long)]
    invert_match: bool,

    #[clap(short, long)]
    regex: bool,

    #[clap(short, long)]
    color: Option<Color>,

    needle: String,

    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{:?}", args);

    // Grabs input
    let input = input::get_input_dyn(args.file)?;
    // println!("{:?}", input);

    let final_lines =
        search::search_lines(input, &args.needle, args.ignore_case, args.invert_match);

    output::print_output(final_lines, args.color, args.needle);
    Ok(())
}
