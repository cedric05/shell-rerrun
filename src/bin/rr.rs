use std::env;
use std::io::{stderr, stdout, Write};
use std::path::Path;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::process::{Command, Stdio};

fn main() {
    let mut rl = Editor::<()>::new();
    let program = match env::args().nth(1) {
        Some(program) => program,
        None => {
            println!("Invalid arguments, run with `rr <command>`");
            return;
        }
    };
    let history_home = format!("{}", shellexpand::tilde("~/.shell-extension"));
    let history_home = Path::new(&history_home);
    let history_location = history_home.join(Path::new(&program).file_name().unwrap());
    if !history_home.exists() {
        std::fs::create_dir_all(&history_home).unwrap();
    }
    let pre = format!("{}: >> ", program);
    loop {
        let readline = rl.readline(&pre);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let args = match shell_words::split(&line) {
                    Ok(args) => args,
                    Err(_) => continue,
                };
                let output = Command::new(&program)
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .output();
                match output {
                    Ok(status) => {
                        if !status.stdout.is_empty() {
                            stdout().write_all(&status.stdout).expect("no stdout");
                        }
                        if !status.stderr.is_empty() {
                            stderr().write_all(&status.stderr).expect("no stderr");
                        }
                    }
                    Err(error) => {
                        println!("error: {error}")
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // incase of incorrect history search, you just want to go back
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(&history_location).unwrap();
}
