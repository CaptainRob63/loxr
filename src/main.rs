use anyhow::Result;
use std::{
    env,
    fs::read_to_string,
    io::{self, Write},
    path::{Path, PathBuf},
    process::exit,
};

mod error;
mod lexer;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let source_path: PathBuf = args[1].clone().into();
            run_file(source_path.as_path())?;
        }

        1 => {
            run_prompt()?;
        }

        _ => {
            print!("usage: loxr [file]?");
            exit(64);
        }
    }

    Ok(())
}

fn run_prompt() -> Result<()> {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }
        run(line.trim_end())?;
        line.clear();
    }

    Ok(())
}

fn run_file(src_path: &Path) -> anyhow::Result<()> {
    let code = read_to_string(src_path)?;
    run(&code)?;

    Ok(())
}

fn run(code: &str) -> Result<()> {
    println!("{code}");
    Ok(())
}
