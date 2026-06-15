pub mod bkg;
use std::env;
use std::process;


fn main() {
    let mut runtime = bkg::init_bkg_set();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("ERROR: Must provide a argument -> [usage]");
        process::exit(1);
    }
    //dbg!(&args);
    let measure: f64 = match args[1].parse::<f64>() {
        Ok(number) => number,
        Err(e) => {
            eprintln!("ERROR: Could not parse {} as a float due to {}, retry", args[1], e);
            process::exit(1);
        }
    };
    
    let max_comb: usize = 3;
    let mut exclusions: Vec<f64> = vec![0.0; 64];
    let ex_size = 0;


    let combinations = runtime.find_combination(measure, max_comb, &mut exclusions, ex_size);
    match combinations {
        Ok(r) => {
            dbg!(r);
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }

}
