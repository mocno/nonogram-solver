use std::time::Instant;

use clap::Parser;

use crate::results::get_mean_rate_painting_nonogram_board;

mod nonogram;
mod results;

/// CLI simples para testas completude de nonogramas de diferentes tamanhos e preenchimento
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 15)]
    size: usize,

    #[arg(short = 'n', long, default_value_t = 50)]
    num_test: usize,

    #[arg(short = 'p', long, default_value_t = 200)]
    num_p_tests: usize,

    #[arg(short = 't', long, default_value_t = false)]
    time: bool,
}

fn print_table_pc(size: usize, num_test: usize, num_p_tests: usize) -> Result<(), ()> {
    let mut rng = rand::rng();
    println!("p;c");
    let ps = (0..=num_p_tests).map(|v| v as f64 / num_p_tests as f64);
    for p in ps {
        let c = get_mean_rate_painting_nonogram_board(&mut rng, size, p, num_test)?;
        println!("{p};{c}");
    }
    Ok(())
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    if args.time {
        let now = Instant::now();
        let result = print_table_pc(args.size, args.num_test, args.num_p_tests);
        let elapsed_time = now.elapsed();
        println!("Time: {:} seconds.", elapsed_time.as_secs());

        result
    } else {
        print_table_pc(args.size, args.num_test, args.num_p_tests)
    }
}
