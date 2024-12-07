use clap::Parser;
use aoc2024::days;

#[derive(Parser)]
struct Args {

    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => days::day01::run(args.part),
        _ => eprintln!("Day {} not implemented", args.day),
    }
}
