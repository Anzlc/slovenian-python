use std::fs::read_to_string;
mod transpiler;
mod file;
use file::write_and_empty;
use transpiler::{ tokenize, transpile };
extern crate getopts;
use getopts::Options;
use std::env;
use std::process::{ exit, Command };

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("t", "transpile", "Create a python file from .spy", "PATH");
    opts.optopt("r", "run", "Run the .spy program", "PATH");
    opts.optopt("p", "preview", "Preview transpiled the .spy program", "PATH");
    opts.optflag("h", "help", "Get help (output this screen)");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|f| {
        println!("An error occured: {f}");
        exit(1);
    });

    if matches.opt_present("h") {
        get_help(opts);
        return;
    }

    if let Some(path) = matches.opt_str("r") {
        run(&path);
    } else if let Some(path) = matches.opt_str("t") {
        transpile_and_write(&path);
        println!("Done");
    } else if let Some(path) = matches.opt_str("p") {
        preview(&path);
        println!("Done");
    } else if args.len() == 2 {
        run(&args[1]);
    }
}

fn run(path: &str) {
    match read_to_string(path) {
        Ok(program) => {
            let tokens = tokenize(&program);
            //println!("{:#?}", tokens);
            let new_program = transpile(&tokens);
            match write_and_empty(&spy_to_py(path), &new_program) {
                Ok(_) => {
                    //Run with python
                    match Command::new("python").arg(&spy_to_py(path)).status() {
                        Ok(_) => {}
                        Err(e) => println!("Error: {e}"),
                    }
                }
                Err(e) => println!("Error while writing file: {e}"),
            }
        }
        Err(e) => println!("Error while reading file: {e}"),
    }
}

fn transpile_and_write(path: &str) {
    match read_to_string(path) {
        Ok(program) => {
            let tokens = tokenize(&program);
            //println!("{:#?}", tokens);
            let new_program = transpile(&tokens);
            match write_and_empty(&spy_to_py(path), &new_program) {
                Ok(_) => {}
                Err(e) => println!("Error while writing file: {e}"),
            }
        }
        Err(e) => println!("Error while reading file: {e}"),
    }
}

fn preview(path: &str) {
    match read_to_string(path) {
        Ok(program) => {
            let tokens = tokenize(&program);
            //println!("{:#?}", tokens);
            let new_program = transpile(&tokens);
            println!("{new_program}");
        }
        Err(e) => println!("Error while reading file: {e}"),
    }
}

fn spy_to_py(path: &str) -> String {
    path.split(".")
        .map(|s| if s == "spy" { "py".to_string() } else { s.to_string() })
        .collect::<Vec<String>>()
        .join(".")
}

fn get_help(opts: Options) {
    let brief = "Usage: piton FILE [options]";
    print!("{}", opts.usage(&brief));
}
