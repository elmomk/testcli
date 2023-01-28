use clap::Parser;
use dirs::home_dir;
use dotenvy::dotenv;
use std::env;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    ///
    mode: String,
    #[arg(short, long, default_value = "")]
    path: String,
    // update: bool,
}

fn ls_path(path: String) {
    println!("ls path: {}", path);
    // println!("home: {}", home);
    Command::new("ls")
        .current_dir(path)
        // .arg(path)
        // .arg(mode2)
        .spawn()
        .expect("terraform  failed");
}

fn main() {
    let args = Cli::parse();
    let my_home = home_dir().unwrap().to_string_lossy().to_string() + "/.runcli/config";
    // println!("my home is: {}", &my_home);
    // try to load environment vairables from three locations
    // load environment variables from .runrsenv
    // load environment variables from scripts/.runrsenv
    // load environment variables from $HOME/.runcli/config
    // load files in order of precedence
    // if .runrsenv exists, load it first and ignore the rest
    // if .runrsenv does not exist, load scripts/.runrsenv
    // if scripts/.runrsenv does not exist, load append my_home with ".runcli/config"
    // if $HOME/.runcli/config does not exist, throw an error
    dotenvy::from_filename(".runrsenv").ok();
    dotenvy::from_filename("scripts/.runrsenv").ok();
    dotenvy::from_filename(my_home).ok();
    dotenv().ok();
    // println!("print: {}", print.unwrap().to_string_lossy().to_string());
    let project_path = if args.path == "" {
        env::var("project_path").unwrap()
    } else {
        args.path
    };
    // .expect("PROJECT_PATH must be set");
    println!("project path is: {}", project_path);
    // print environment variables
    // println!("env: {:?}", std::env::vars().collect::<Vec<_>>());

    println!("tf mode is: {}", args.mode);
    println!("path: {}", project_path);

    ls_path(project_path.to_string())
}
