pub mod bkg;
pub mod cli;
use clap::Parser;
use std::process;

fn main() {
    let mut runtime = bkg::init_bkg_set();
    let cli = cli::Cli::parse();

    let mut max_comb: usize = 2;

    let measure = match Some(cli.measure) {
        Some(float) => float,
        None => {
            eprintln!("ERROR: No value provided");
            process::exit(1);
        }
    };
    if let Some(max_combinations) = cli.max_combinations {
        max_comb = max_combinations;
    }
    if let Some(ex) = cli.exclusions {
        runtime.set_exclusions(ex.to_vec());
    }

    let combinations = runtime.find_combination(measure, max_comb);
    for (i, combination) in combinations.iter().enumerate() {
        print!("{}) ", i + 1);
        if combination.0 != bkg::ErrorType::None {
            println!("{}", combination.0);
            continue;
        }
        for (j, block) in combination.1.iter().enumerate() {
            if bkg::f_comp(*block, 0.0) {
                continue;
            };
            if j == 0 {
                print!("| {:.4} |", block);
            } else {
                print!(" {:.4} |", block);
            }
        }
        println!();
    }
}
