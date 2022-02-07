use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt)]
struct Argumentos {
    #[structopt(default_value="KK eae mein")]
    /// O que o gato diz?
    mensagem: String,

    #[structopt(short = "d", long = "dead")]
    /// Faz o gato aparecer morto
    dead: bool,
}

fn main() {
    let args = Argumentos::from_args();
    let msg = args.mensagem;
    let olhos = if args.dead { "x" } else { "o" };
    let woofies = Regex::new(r"([w]+)([ou]+)([f]+)").unwrap();
    if woofies.is_match(&msg.to_lowercase()) {
        eprintln!("O gato não devia latir como um cachorro!.");
    }
    println!("{}", msg);
    println!(" \\");
    println!("  \\");
    println!("      /\\_/\\");
    println!("     ( {0} {0} )", olhos);
    println!("     =( I )=");
}
