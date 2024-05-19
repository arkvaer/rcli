use std::path::Path;
use clap::Parser;

// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
}


#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or Covert VSV to other formats")]
    Csv(CsvOpts)
}

#[derive(Debug, Parser)]
struct CsvOpts {
    // , value_parser = verify_input_file
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}


fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts)
}


fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}