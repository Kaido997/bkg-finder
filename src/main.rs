pub mod bkg;
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        eprintln!("ERROR: Must provide a argument -> [usage]")
    }
    dbg!(&args);
    let mut _max_comb: u8 = 2;
    let measure: f64 = match args[1].parse::<f64>() {
        Ok(number) => number,
        Err(e) => {
            eprintln!("ERROR: Could not parse {} as a float due to {}, retry", args[1], e);
            process::exit(1);
        }
    };

    bkg::init_bkg_set();
    println!("The float is: {}", measure);
}
