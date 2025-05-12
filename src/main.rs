use std::{
    fs::OpenOptions,
    io::{self, Result, Write}, 
};

fn draw_ascii() {
    println!(r#"
             |             |           |      
  _ \   __|  __ \    _ \   __|   _` |  |   _ \
  __/  (     | | |  (   |  |    (   |  |   __/
\___| \___| _| |_| \___/  \__| \__,_| _| \___|
    "#);
}

fn print_now(prompt: &str) -> Result<String> {
    let mut stdout = io::stdout();

    write!(stdout, "{}", prompt)?;
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn sleeep(time: u64) {
    std::thread::sleep(std::time::Duration::from_millis(time))
}

fn main() -> Result<()> {
    let mut logger = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/echotale_history.txt")?;

    draw_ascii();
    sleeep(900);
    println!("Wait a moment");
    sleeep(2500);


    loop {
        let user_output = print_now(r#"type (for quit ":quit"): "#)?;

        match user_output.as_str() {
            ":quit" => {
                break
            }
            _ => {
                println!("You typed: {}", user_output);
                writeln!(logger, "{}", user_output.trim())?;
            },
        }
    }
    println!("file path: /tmp/echotale_history.txt");
    Ok(())
}
