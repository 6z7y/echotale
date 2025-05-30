use std::{
    fs::OpenOptions,
    io::Write
};

pub const FILE_PATH: &str = "/tmp/echotale_history.txt";

pub fn logger(user_output: &str) {
    let mut logfile = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_PATH).unwrap();

    println!("You weote: {}", user_output);
    writeln!(logfile, "{}", user_output).unwrap();
}
