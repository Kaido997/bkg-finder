use bkg_finder::bkg;
use std::env;
use std::process;
use std::time::{Duration, Instant};

const EPSILON: f64 = 0.00001;

const ALL_MEASURES: &[f64] = &[
    0.0500, 0.1000, 0.2000, 0.2500, 0.5000, 0.7500, 0.8000, 1.0000, 2.0000, 3.0000, 4.0000,
];

fn validate(combo: &[f64], measure: f64) -> bool {
    let sum: f64 = combo.iter().take_while(|&&x| x > EPSILON).sum();
    (sum - measure).abs() < EPSILON
}

fn run_bench(count: usize) -> (usize, usize, Duration) {
    let mut ok: usize = 0;
    let mut fail: usize = 0;

    let start = Instant::now();
    for i in 0..count {
        let measure = ALL_MEASURES[i % ALL_MEASURES.len()];

        let mut rt = bkg::init_bkg_set();
        let mut exclusions = vec![0.0; 64];
        match rt.find_combination(measure, 1, &mut exclusions, 0) {
            Ok(combinations) if validate(&combinations[0], measure) => ok += 1,
            _ => fail += 1,
        }
    }
    let elapsed = start.elapsed();

    (ok, fail, elapsed)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: bench_rust <count>");
        process::exit(1);
    }
    let count: usize = args[1].parse().expect("count must be integer");

    let (ok, fail, elapsed) = run_bench(count);
    println!(
        "count={count} ok={ok} fail={fail} elapsed_ms={}",
        elapsed.as_millis()
    );
}
