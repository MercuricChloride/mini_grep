#[allow(unused)]

pub mod logic {
    use std::{env, fs};

    fn print_line_numbers(file: &str, query: &str) {
        file.lines()
            .enumerate()
            .for_each(|(index, line)| {
                if line.contains(query){
                    println!("{} {}", index+1, line);
                }
            });
    }

    fn count_matches(file: &str, query: &str) {
        let count = file.split_ascii_whitespace()
            .fold(0, |acc, word| {
                if word.contains(query){
                    acc+1
                } else {
                    acc
                }
            });
        println!("Total matches found: {}", count);
    }

    pub struct Cli {
        cmd: String,
        pattern: String,
        filepath: String,
    }

    pub fn handle_inputs() -> Cli {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 4 {
            Cli {
                cmd: String::from(&args[1]),
                pattern: String::from(&args[2]),
                filepath: String::from(&args[3]),
            }
        } else {
            panic!("Invalid input!")
        }
    }

    pub fn exec(cli: Cli) {
        let file = fs::read_to_string(&cli.filepath).unwrap();
        match &file.contains(&cli.pattern) {
            true => {
                match &cli.cmd[..] {
                    "count" => {
                        count_matches(&file, &cli.pattern);
                    }, 
                    "print" => {
                        print_line_numbers(&file, &cli.pattern);
                    },
                    _=> {
                        println!("Invalid command input");
                    }
                }
            },
            false => {
                println!("No matches found for file: {}", &cli.filepath);
            }
        }
    }

}
