use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Result, Write} 
};

mod structfn;
use crate::structfn::{
    EchoSystem,
    Opration,
    Time
};

const FILE_PATH: &str = "/tmp/echotale_history.txt";

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
        if user_output.is_empty() {
            println!("empty text!");
            continue;
        } else {
            println!("You weote: {}", user_output);
            writeln!(logger, "{}", user_output)?;

            if user_output == ":calculate" {
                println!("\nHint: '2 * 5'\n");
                let prompt = EchoSystem::print_new("cacl> ")?;

                let parts: Vec<&str> = prompt.split_whitespace().collect();
                if parts.len() != 3 {
                    println!("type like (3 + 4)");
                    continue;
                }

                let a = match parts[0].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("first number invalid");
                        continue;
                    }
                };

                let op = match Opration::from_str(parts[1]) {
                    Some(op) => op,
                    None => {
                        println!("error type (+ - * /)");
                        continue;
                    }
                };

                let b = match parts[2].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("second number invalid");
                        continue;
                    }
                };

                let result = op.apply(a, b);
                println!("= {}", result)
            } else if user_output == ":clear" {
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
            } else if user_output == ":time" {
                println!("\nHint: '23:59'\n");
                let prompt_input = match EchoSystem::print_new("time> ") {
                    Ok(input) => input,
                    Err(e) => {
                        println!("Input error: {}", e);
                        String::new()
                    }
                };

                match Time::from_str(&prompt_input) {
                    Ok(time) => {
                        time.show();
                        println!("Total minute: {}", time.to_minute());
                        println!("Total secound: {}", time.to_second());
                    }
                    Err(msg) => println!("invlid format: {}", msg)
                }
                
            } else if user_output == ":quit" {
                break;
            } else if user_output == ":?" {
                println!("
                \nAvailable commands:\n\
                    :calculate  - Calculate menu\n\
                    :clear      - Clear history\n\
                    :show       - Show history\n\
                    :search     - Search history\n\
                    :time       - Time System\n\
                    :quit       - Exit the program\n\
                    :?          - Show this help message\n\
                    ")
            }
        }
    }
    print!("file path: /tmp/echotale_history.txt");
    Ok(())
}
