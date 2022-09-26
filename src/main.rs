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

fn print_authors() {
    println!("Autores deste projeto e membros do grupo RUST TASKFORCE !!!");
    println!("Ana Vieira de Souza (17100535)");
    println!("Gabriel Simonetto (18103862)");
    println!("Matheus D C Roque (19100538)");
    println!("Paloma Cione (17100530)");
}

fn main() {
    let args = Args::parse();

    print_authors();

    println!("Parsing {}", &args.filename);
    let result = parse_into_symbol_table(&args.filename);
    print_symbol_table(result);
}
