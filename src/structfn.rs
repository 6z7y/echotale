use std::io::{self, Write};

pub struct EchoSystem;

pub enum Opration {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub struct Time {
    pub hour: u32,
    pub minute: u32,
}

impl EchoSystem {
    pub fn draw_ascii() {
        println!(r#"
                 |             |           |      
      _ \   __|  __ \    _ \   __|   _` |  |   _ \
      __/  (     | | |  (   |  |    (   |  |   __/
    \___| \___| _| |_| \___/  \__| \__,_| _| \___|
        "#);
    }

    pub fn print_new(prompt: &str) -> io::Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    pub fn sleeep(time: u64) {
        std::thread::sleep(std::time::Duration::from_millis(time))
    }
}

impl Time {
    pub fn from_str(text: &str) -> Result<Self, String> {
        let parts: Vec<&str> = text.split(':').collect();
        if parts.len() != 2 {
            return Err("Error0".to_string());
        }
        
        let hour = parts[0].parse::<u32>().map_err(|_| "inviled hour".to_string())?;
        let minute = parts[1].parse::<u32>().map_err(|_| "inviled minute".to_string())?;

        if hour > 23 || minute > 59 {
            return Err("Hour must be 0-23, and minute 0-59".to_string());
        }

        Ok(Self { hour, minute })
    }

    pub fn to_minute(&self) -> u32 {
        self.hour * 60 + self.minute
    }

    pub fn to_second(&self) -> u32 {
        (self.hour * 60 + self.minute) * 60
    }

    pub fn show(&self) {
        println!("{:02}:{:02}", self.hour, self.minute);
    }
}

impl Opration {
    pub fn from_str(op: &str) -> Option<Self> {
        match op {
            "+" => Some(Opration::Add),
            "-" => Some(Opration::Subtract),
            "*" => Some(Opration::Multiply),
            "/" => Some(Opration::Divide),
            _ => None
        }
    }

    pub fn apply(&self, a: f64, b: f64) -> f64 {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Multiply => a * b,
            Self::Divide => {
                if b == 0.0 {
                    println!("Cannot Divide by zero!")
                }
                a / b
            }
        }
    }
}
