use std::{
    io::{self, Result, Write}
};

pub struct EchoSystem;

impl EchoSystem {
    pub fn draw_ascii() {
        println!(r#"
                 |             |           |      
      _ \   __|  __ \    _ \   __|   _` |  |   _ \
      __/  (     | | |  (   |  |    (   |  |   __/
    \___| \___| _| |_| \___/  \__| \__,_| _| \___|
        "#);
    }

    pub fn print_now(prompt: &str) -> Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input)
    }

    pub fn sleeep(time: u64) {
        std::thread::sleep(std::time::Duration::from_millis(time))
    }
}
