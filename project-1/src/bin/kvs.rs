use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opt {
    #[clap(subcommand)]
    pub cmd: Command,
}

//#[derive(Parser)]
#[derive(Debug, Subcommand)]
enum Command {
    #[clap(about = "Set the value of a string key to a string")]
    Set(Set),
    #[clap(about = "Get the string value of a given string key")]
    Get(Get),
    #[clap(about = "Remove a given key")]
    Remove(Remove),
}

#[derive(Debug, Parser)]
struct Set {
    key: String,
    value: String,
}

#[derive(Debug, Parser)]
struct Get {
    key: String,
}

#[derive(Debug, Parser)]
struct Remove {
    key: String,
}

fn main() {
    let opt = Opt::parse();

    match opt.cmd {
        Command::Set(_set) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Get(_get) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Remove(_rm) => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
