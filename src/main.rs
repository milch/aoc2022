pub mod solutions;

use clap::arg;
use clap::Parser;
use solutions::*;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    day: u8,
}

fn main() {
    let args = Cli::parse();
    match args.day {
        1 => day_01::print_solution(),
        2 => day_02::print_solution(),
        3 => day_03::print_solution(),
        4 => day_04::print_solution(),
        5 => day_05::print_solution(),
        6 => day_06::print_solution(),
        _ => unimplemented!(),
    }
}