use std::process::Command;
use clap::Parser;
use std::process::Child;

#[derive(Parser)]
struct Cli {
    /// 
    mode: String,
    mode2: String,
    // #[arg(short, long)]
    // update: bool,
}

fn tf_simple(mode: String) -> Child {
    Command::new("echo")
            .arg(mode)
            .arg(mode2)
            .spawn()
            .expect("terraform  failed")
}

fn main() {
    let args = Cli::parse();

    println!("tf mode is: {}", args.mode);
    // println!("update : {}", args.update);

    tf_simple(args.mode);
}
