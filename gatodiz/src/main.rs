use std::io::{self, Read};
use structopt::StructOpt;
use regex::Regex;
use colored::*;

#[derive(StructOpt)]
struct Argumentos {
    #[structopt(default_value="KK eae mein")]
    /// O que o gato diz?
    mensagem: String,

    #[structopt(short = "d", long = "dead")]
    /// Faz o gato aparecer morto
    dead: bool,

    #[structopt(short = "i", long = "stdin")]
    /// Faz o gato aparecer morto
    stdin: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Carregar o gato de um arquivo
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Argumentos::from_args();
    let mut msg = String::new();
    if args.stdin {
        io::stdin().read_to_string(&mut msg)?;
    } else {
        msg = args.mensagem;
    }
    let olhos = if args.dead { "x" } else { "o" };
    let woofies = Regex::new(r"([w]+)([ou]+)([f]+)").unwrap();
    if woofies.is_match(&msg.to_lowercase()) {
        eprintln!("O gato não devia latir como um cachorro!.");
    }
    println!("{}", msg.bright_yellow().underline().on_purple());
    match &args.catfile {
        Some (path) => {
            let template = std::fs::read_to_string(path)?;
            let cat = template.replace("{eye}", olhos);
            println!("{}", &cat);
        },
        None => {
            println!(" \\");
            println!("  \\");
            println!("      /\\_/\\");
            println!("     ( {0} {0} )", olhos.red().bold());
            println!("     =( I )=");
        },
    }
    Ok(())
}
