use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The measure in which the combination is calculated
    pub measure: f64,

    /// Sets the maximum combination
    #[arg(short, long)]
    pub max_combinations: Option<usize>,

    /// Value to exclude from the combinations e.g. missing blocks
    #[arg(short, long, num_args = 1..)]
    pub exclusions: Option<Vec<f64>>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}
