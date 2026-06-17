pub mod bkg;
pub mod cli;
use clap::Parser;
use std::process;



fn main() {
    let mut runtime = bkg::init_bkg_set();
    let cli = cli::Cli::parse();

    let mut max_comb: usize = 2;
    let mut exclusions: Vec<f64> = vec![0.0; 64];
    let mut ex_size = 0;

    let measure = match Some(cli.measure) {
        Some(float) => float,
        None => {eprintln!("ERROR: No value provided"); process::exit(1);}
    };
    if let Some(max_combinations) = cli.max_combinations {
        max_comb = max_combinations;
    }
    if let Some(ex) = cli.exclusions {
        for (i, item) in ex.to_vec().iter().enumerate() {
            ex_size += 1;
            exclusions[i] = *item;
        }
    }

    let combinations = runtime.find_combination(measure, max_comb, &mut exclusions, ex_size);
    match combinations {
        Ok(r) => {
            for (i, combination) in r.iter().enumerate() {
                print!("{}) ", i + 1);
                for (j, block) in combination.iter().enumerate() {
                    if bkg::f_comp(*block, 0.0) { continue };
                    if j == 0 {
                        print!("| {:.4} |", block);
                    } else if j ==  combination.len() - 1 {
                        print!(" {:.4} |", block);
                    } else {
                        print!(" {:.4} |", block);
                    }
                }
                print!("\n");
            }
        },
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}
