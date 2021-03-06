use std::{env, path::PathBuf, process};

mod models;
mod parser;
mod updater;
mod util;
mod writer;
// mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let (mut dataset, mut meta, mut sc, mut tc) = parser::run(&config).unwrap();
    updater::run(&mut dataset, &mut meta, &mut sc, &mut tc);
    writer::run(&config, &dataset, &meta);

    // println!("SOURCE CLIQUES");
    // util::print::cliques_string(&sc, &stuff.dict);
    // println!("");
    // println!("TARGET CLIQUES");
    // util::print::cliques_string(&tc, &stuff.dict);

    // // println!("");
    // // println!("TRIPLES");
    // // util::print::triples_string(&stuff.triples, &stuff.dict);
}

pub struct Config {
    dataset_path: PathBuf,
    meta_folder_path: PathBuf,
    update_path: PathBuf,
    use_fast: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 1 || args[1] == "--help" || args[1] == "-h" {
            println!("STFU LOSER BITCH");
            process::exit(0);
        }

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let dataset_path = PathBuf::from(&args[1]);
        if !dataset_path.exists() {
            return Err("dataset path does not exist");
        }

        let update_path = PathBuf::from(&args[2]);
        if !update_path.exists() {
            return Err("update path does not exist");
        }

        let meta_folder_path = PathBuf::from(&args[3]);

        let mut use_fast = false;
        if args.len() > 4 && (args[4] == "--fast" || args[4] == "-f") {
            println!("[ANON] GAMER MODE ACTIVATED _  _ _ xX_Using fast mode_Xx");
            use_fast = true;
        }

        if use_fast && meta_folder_path.exists() {
            return Err("Using fast mode and meta folder path already exists");
        } else if !use_fast && !meta_folder_path.exists() {
            return Err("using slow mode and meta folder path does not exist");
        }

        Ok(Config {
            dataset_path,
            meta_folder_path,
            update_path,
            use_fast,
        })
    }
}
