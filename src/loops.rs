use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Result} 
};

use crate::{
    log::{FILE_PATH, logger},
    structfn::{EchoSystem, Opration, Student, Time, Permission, User}
};

pub fn loops() -> Result<()> {
    loop {
        let user_output = EchoSystem::print_new("type (':?' for help): ")?;

        if user_output.is_empty() {
            println!("empty text!");
            continue;
        } else {
            logger(&user_output);

            if user_output == ":calculate" {
                loop {
                    println!("\nHint: '2 * 5'\n");
                    let prompt = EchoSystem::print_new("cacl> ")?;

                    if prompt == ":quit" {
                        break;
                    }

                    let parts: Vec<&str> = prompt.split_whitespace().collect();
                    if parts.len() != 3 {
                        println!("type like (3 + 4)");
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
                }
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
                let results: HashSet<_> = reader
                    .lines()
                    .map_while(Result::ok)
                    .filter(|line| line.contains(&keyword))
                    .collect();

                // Step 4: print results
                if results.is_empty() {
                    println!("No matching results.");
                } else {
                    // println!("Found {} result(s):", results.len());
                    for line in results {
                        println!("- {}", line);
                    }
                } 
            } else if user_output == ":student" {
                let mut studints: Vec<Student> = Vec::new();

                loop {
                    let name = EchoSystem::print_new("enter student name (or 'exit' for stop): ")?;
                    if name.eq_ignore_ascii_case("exit") {
                        break;
                    }

                    println!("Enter grade for {}:", name);
                    let grade_input = EchoSystem::print_new("")?;
                    let grade: u8 = match grade_input.parse() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Invalid grade, Please enter a number between 0-100.");
                            continue;
                        }
                    };

                    studints.push(Student { name: name.to_string(), grade });
                    println!("Student added!\n");
                }

                println!("\n--- Student Reports ---");
                for student in studints {
                    student.check_student();
                }


            } else if user_output == ":time" {
                loop {
                    println!("\nHint: '23:59'\n");
                    let prompt_input = match EchoSystem::print_new("time> ") {
                        Ok(input) => input,
                        Err(e) => {
                            println!("Input error: {}", e);
                            String::new()
                        }
                    };

                    if prompt_input == ":quit" {
                        break;
                    }

                    match Time::from_str(&prompt_input) {
                        Ok(time) => {
                            time.show();
                            println!("Total minute: {}", time.to_minute());
                            println!("Total secound: {}", time.to_second());
                        }
                        Err(msg) => println!("invlid format: {}", msg)
                    }
                }
            } else if user_output == ":user" {
                let mut users: Vec<User> = Vec::new();

                loop {
                    let command = EchoSystem::print_new("\nType a command (add, list, select, toggle, exit): ")?;

                    match command.as_str() {
                        "add" => {
                            // Get user name
                            let name = EchoSystem::print_new("Enter name: ")?;

                            // Get permission.
                            let perm_input = EchoSystem::print_new("Enter permission (root/user/guest): ")?;

                            let permission = match perm_input.as_str() {
                                "root" => Permission::Root,
                                "user" => Permission::User,
                                "guest" => Permission::Guest,
                                _ => {
                                    println!("Invalid permisiion!");
                                    continue;
                                }
                            };

                            let user = User::new(&name, permission);
                            users.push(user);
                            println!("User added.");
                        }
                        "toggle" => {
                            let name = EchoSystem::print_new("Enter user name to toggle active: ")?;
                            let mut found = false;

                            for user in &mut users {
                                if user.name == name {
                                    user.toggle_active();
                                    println!("Toggled user '{}' Now active: {}", user.name, user.is_active);
                                    found = true;
                                    break;
                                }
                            }

                            if !found {
                                println!("User not found.");
                            }
                        }
                        "list" => {
                            println!("\n--- All Users ---");
                            User::list_users(&users);
                        }
                        "select" => {
                            let perm_input = EchoSystem::print_new("Select permisiion to show users (root/user/guest): ")?;

                            let selected_permision = match perm_input.as_str() {
                                "root" => Permission::Root,
                                "user" => Permission::User,
                                "guest" => Permission::Guest,
                                _ => {
                                    println!("Invalid permission");
                                    continue;
                                }
                            };

                            println!("Users with {:?} permission:", selected_permision);
                            let mut found = false;
                            for user in &users {
                                if user.permission == selected_permision {
                                    user.info();
                                    found = true;
                                }
                            }
                            if !found {
                                println!("No users found with that permission.");
                            }
                        }
                        "exit" => {
                            println!("Exiting...");
                            break;
                        }
                        _ => println!("unknown command")
                    }
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
                    :student    - Result students\n\
                    :time       - Time System\n\
                    :user       - Make list user\n\
                    :quit       - Exit the program\n\
                    :?          - Show this help message\n\
                ")
            }
        }
    }
    Ok(())
}
