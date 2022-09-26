use clap::Parser;
use trabalho::lexical::{parse_into_symbol_table, print_symbol_table};

/// Basic parser
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the file with code to be parsed
    #[clap(short, long, value_parser)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    println!("Parsing {}", &args.filename);
    let result = parse_into_symbol_table(&args.filename);
    print_symbol_table(result);
}
