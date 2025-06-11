use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Result} 
};

use crate::{
    func::{
        Adventurer, 
        EchoSystem, 
        MagicGem, 
        Operator,
        Permission, 
        Rarity,
        Student,
        Status,
        Task,
        Time,
        User
    }, 
};

use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, DefaultHinter, DefaultPrompt, Emacs, FileBackedHistory, KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent, ReedlineMenu, Signal
};

use {
  nu_ansi_term::{Color, Style},
};

const FILE_PATH: &str = "/tmp/echotale_history.txt";
pub fn loops() -> Result<()> {
    let history = Box::new(
      FileBackedHistory::with_file(1000, "/tmp/echotale_history.txt".into())
        .expect("Error configuring history with file"),
    );

    let commands = vec![
        ":calculate".into(),
        ":clear".into(),
        ":gem".into(),
        ":show".into(),
        ":search".into(),
        ":student".into(),
        ":task".into(),
        ":time".into(),
        ":user".into(),
        ":quit".into(),
        ":?".into(),
    ];

    let completer = Box::new(DefaultCompleter::new(commands.clone()));
    
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
    // Set up the required keybindings
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );    // إضافة اختصار shift+tab للتراجع
          //
    keybindings.add_binding(
        KeyModifiers::SHIFT,
        KeyCode::BackTab,
        ReedlineEvent::MenuPrevious,
    );

    let edit_mode = Box::new(Emacs::new(keybindings));

    let mut line_editor = Reedline::create()
        .with_history(history)
        .with_completer(completer)
        .with_menu(ReedlineMenu::HistoryMenu(completion_menu))
        .with_edit_mode(edit_mode)
        .with_hinter(Box::new(
            DefaultHinter::default()
            .with_style(Style::new().italic().fg(Color::LightGray)),
        ));
    let prompt = DefaultPrompt::default();

    println!("type (':?' for help):");

    loop {
        let user_output = match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => buffer,
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nExiting...");
                break;
            }
            _ => continue,
        };

        if user_output == ":calculate" {
            println!("Enter expression (5 * 3) or 'exit'");
            loop {

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let trimmed = input.trim();

                if trimmed.eq_ignore_ascii_case("exit") {
                    break
                }

                let parts: Vec<&str> = trimmed.split_whitespace().collect();

                if parts.len() != 3 {
                    println!("Invalid format. Use number operator number");
                    continue;
                }

                let num1 = parts[0].parse::<i32>();
                let op = Operator::parse_operator(parts[1]);
                let num2 = parts[2].parse::<i32>();

                match (num1, op, num2) {
                    (Ok(a), Some(op), Ok(b)) => {
                        match op {
                            Operator::Add => println!("Result: {} + {} = {}", a, b, a + b),
                            Operator::Sub => println!("Result: {} - {} = {}", a, b, a - b),
                            Operator::Mul => println!("Result: {} * {} = {}", a, b, a * b),
                            Operator::Div => {
                                if b == 0 {
                                    println!("cannot divide by zero");
                                    continue;
                                } else {
                                    println!("Result: {} / {} = {}", a, b, a / b)
                                }
                            }
                        }
                    }
                    _ => println!("Error operator")
                }
            }
        } else if user_output == ":clear" {
            std::fs::write(FILE_PATH, "")?;
            println!("History cleared.");
        } else if user_output == ":gem" {
            let name = EchoSystem::print_new("enter your name: ");
            let mut hero = Adventurer::new(name?.as_str());

            let gem_count_input = EchoSystem::print_new("howw much gem you want to catch? ");
            let gem_count: u32 = gem_count_input?.parse::<u32>().unwrap_or(0);

            for i in 1..gem_count {
                println!("\n--- gem number {} ---", i);
                let color = EchoSystem::print_new("color gem: ")?;
                let power_input = EchoSystem::print_new("power gem (number): ");
                let power: u32 = power_input?.parse().unwrap_or(0);

                let rarity_input = EchoSystem::print_new("gem (common / Rare / Legendary / Epic): ");
                let rarity = match rarity_input?.to_lowercase().as_str() {
                    "common" => Rarity::Common,
                    "rare" => Rarity::Rare,
                    "legendary" => Rarity::Legendary,
                    "epic" => Rarity::Epic,
                    _ => {
                        println!("error input, default common");
                        Rarity::Common
                    }
                };

                let gem = MagicGem {
                    color,
                    power,
                    rarity
                };

                hero.collect_gem(gem);
            }
            hero.report();
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

        } else if user_output == ":task" {

            let mut tasks: Vec<Task> = Vec::new();

            let count_input = EchoSystem::print_new("how much task you want? ")?;
            let count: u32 = count_input.parse().unwrap_or(0);

            for i in 0..count {
                println!("\n--- task num {} ---", i + 1);
                let title = EchoSystem::print_new("enter title: ")?;
                let description = EchoSystem::print_new("description of task: ")?;
                let priority_input = EchoSystem::print_new("prioity task (num): ")?;
                let priority: u8 = priority_input.parse().unwrap_or(1);

                let task = Task {
                    title,
                    description,
                    priority,
                    status: Status::Pending
                };
                tasks.push(task);
            }

            println!("\n--- list task ---");
            for (i, task) in tasks.iter().enumerate() {
                println!("{}) {} - status: {:?}", i, task.title, task.status);
            }

            let index_input = EchoSystem::print_new("Enter task number to update (0 or 1): ")?;
            let index: usize = index_input.parse().unwrap_or(0);

            if let Some(task) = tasks.get_mut(index) {
                let status_input = EchoSystem::print_new("Enter new status (pending / inprogress / completed): ")?;
                task.status = match status_input.to_lowercase().as_str() {
                    "pending" => Status::Pending,
                    "inprogress" => Status::InProgress,
                    "completed" => Status::Completed,
                    _ => {
                        println!("Invalid input. Keeping old status.");
                        task.status.clone()
                    }
                };
            } else {
                println!("no task in this number.");
            }

            println!("\nUpdated Task:");
            for task in &tasks {
                println!("Task: {}, Status: {:?}", task.title, task.status);
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
                :gem        - catch stone\n\
                :show       - Show history\n\
                :search     - Search history\n\
                :student    - Result students\n\
                :task       - add tasks\n\
                :time       - Time System\n\
                :user       - Make list user\n\
                :quit       - Exit the program\n\
                :?          - Show this help message\n\
            ")
        }
    }
    Ok(())
}

