use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filepath = &args[2];
    let file = fs::read_to_string(filepath).unwrap();

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


    match &file.contains(query) {
        true => {
            print_line_numbers(&file, query);
            count_matches(&file, query);
        },
        false => {
            println!("No matches found for file: {}", filepath);
        }
    }
}
