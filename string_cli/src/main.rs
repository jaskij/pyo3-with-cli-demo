use clap::Parser;
use pyo3::PyErr;
use string_sum::sum_as_string;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// first number
    a: usize,

    /// second number
    b: usize,
}

fn main() -> Result<(), PyErr>{
    let cli = Cli::parse();

    let sum = sum_as_string(cli.a, cli.b)?;

    println!("{sum}");

    Ok(())
}
