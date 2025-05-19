use std::{
    fs::{OpenOptions, File},
    io::{BufRead, BufReader, Result, Write}, 
};

const FILE_PATH: &str = "/tmp/echotale_history.txt";

mod structfn;
use crate::structfn::EchoSystem;

fn main() -> Result<()> {
    let mut logger = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_PATH)?;

    EchoSystem::draw_ascii();
    EchoSystem::sleeep(900);
    println!("Wait a moment");
    EchoSystem::sleeep(2500);


    loop {
        let user_output = EchoSystem::print_new("type (':?' for help): ")?;

        if !user_output.is_empty() {
            if user_output == ":clear" {
                std::fs::write(FILE_PATH, "")?;
                println!("History cleared.");
            } else if user_output == ":show" {
                let result = std::fs::read_to_string(FILE_PATH)?;

                if result.trim().is_empty() {
                    println!("History is empty.");
                } else {
                    println!("History:");
                    for (i, line) in result.lines().enumerate() {
                        println!("{}: {}", i + 1, line);
                    }
                }
            } else if user_output == ":search" {
                // Step 1: Ask for search keyword
                let keyword = EchoSystem::print_new("Search keyword: ")?;

                // Step 2: Open the file for reading
                let file = File::open(FILE_PATH)?;
                let reader = BufReader::new(file);

                // Step 3: Read lines and filter
                let results: Vec<_> = reader
                    .lines()
                    .filter_map(Result::ok)
                    .filter(|line| line.contains(&keyword))
                    .collect::<Vec<_>>();

                // Step 4: print results
                if results.is_empty() {
                    println!("No matching results.");
                } else {
                    println!("Found {} result(s):", results.len());
                    for line in results {
                        println!("- {}", line);
                    }
                } 
            } else if user_output == ":quit" {
                break;
            } else if user_output == ":?" {
                println!("
                \nAvailable commands:\n\
                    :clear      - Clear history\n\
                    :show       - Show history\n\
                    :search     - Search history\n\
                    :quit       - Exit the program\n\
                    :?          - Show this help message\n\
                    ")
            } else {
                println!("You weote: {}", user_output);
                writeln!(logger, "{}", user_output)?;
            }
        } else {
            println!("empty text!");
        }
    }
    print!("file path: /tmp/echotale_history.txt");
    Ok(())
}
