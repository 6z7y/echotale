use std::{
    fs::OpenOptions,
    io::{Result, Write}, 
};

mod structfn;

use crate::structfn::EchoSystem;

fn main() -> Result<()> {
    let mut logger = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/echotale_history.txt")?;

    EchoSystem::draw_ascii();
    EchoSystem::sleeep(900);
    println!("Wait a moment");
    EchoSystem::sleeep(2500);


    loop {
        let user_output = EchoSystem::print_now("type (':quit' for exit): ")?;

        if user_output.is_empty() {
            println!("is empty text");
        } else if user_output == ":quit" {
            break;
        } else {
            println!("You weote: {}", user_output);
            writeln!(logger, "{}", user_output)?;
        }
    }
    print!("file path: /tmp/echotale_history.txt");
    Ok(())
}
