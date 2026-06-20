use crate::glitch;
use std::io::{self, Write};
use crate::filesystem::FileSystem;

pub fn run() {
    let mut fs = FileSystem::new();
    let stdin = io::stdin();

    loop {
        print!("{}", fs.prompt());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];
        let arg = parts.get(1).copied().unwrap_or("");

        match command {
            "help" => {
                println!();
                println!("  available commands:");
                println!("    ls              list current directory");
                println!("    cd <dir>        change directory");
                println!("    cat <file>      read a file");
                println!("    run <file.vsk>  execute a VØSK program");
                println!("    clear           clear the screen");
                println!("    exit            close the terminal");
                println!();
            }

            "ls" => {
                let dir = fs.current_dir();
                let mut entries: Vec<&String> = dir.keys().collect();
                entries.sort();
                println!();
                for entry in entries {
                    println!("  {}", entry);
                }
                println!();
            }

            "cd" => {
                if arg.is_empty() {
                    println!("usage: cd <directory>");
                } else {
                    match fs.change_dir(arg) {
                        Ok(_) => {}
                        Err(e) => println!("error: {}", e),
                    }
                }
            }

            "cat" => {
                if arg.is_empty() {
                    println!("usage: cat <file>");
                } else {
                    match fs.get_file(arg) {
                        Some(contents) => {
                            println!();
                            println!("{}", contents);
                            println!();
                        }
                        None => println!("error: '{}' not found", arg),
                    }
                }
            }

            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
            }

            "run" => {
                            if arg.is_empty() {
                                println!("usage: run <file.vsk>");
                            } else {
                                match fs.get_file(arg) {
                                    Some(contents) => {
                                        use std::process::Command;
                                        use std::fs;
            
                                        let tmp = "/tmp/vosk_tmp.vsk";
                                        fs::write(tmp, contents).unwrap();
            
                                        let output = Command::new("vosk")
                                            .arg("run")
                                            .arg(tmp)
                                            .output();
            
                                        match output {
                                            Ok(out) => {
                                                println!();
                                                print!("{}", String::from_utf8_lossy(&out.stdout));
                                                println!();
                                            }
                                            Err(_) => {
                                                println!("error: vosk binary not found");
                                            }
                                        }
                                    }
                                    None => println!("error: '{}' not found", arg),
                                }
                            }
                        }
            
            "exit" => {
                println!("session closed.");
                break;
            }

            _ => {
                println!("unknown command: '{}'. type 'help' for available commands.", command);
            }
        }
        glitch::glitch_line();
    }
}
