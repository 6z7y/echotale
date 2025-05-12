use std::{
    fs::OpenOptions,
    io::{self, Result, Write}
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
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn sleeep(time: u64) {
    std::thread::sleep(std::time::Duration::from_millis(time))
}

fn main() -> Result<()> {
    draw_ascii();
    sleeep(900);
    println!("Wait a moment");
    sleeep(2500);

    let mut logger = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/echotale_history.txt")?;

    loop {
        let user_output = print_now(r#"type (for quit ":quit"): "#)?;

        match user_output.trim() {
            ":quit" => {
                println!("User Quit...");
                writeln!(logger, "{}", user_output.trim())?;
                break
            }
            _ => writeln!(logger, "{}", user_output.trim())?,
        }
        println!("{}", user_output);
    }
    Ok(())
}


