use std::io::{self, Write};

#[derive(Debug)]
pub enum Rarity {
    Common,
    Rare,
    Legendary,
    Epic
}

#[derive(Debug)]
pub struct MagicGem {
    pub color: String,
    pub power: u32,
    pub rarity: Rarity
}

#[derive(Debug)]
pub struct Adventurer {
    pub name: String,
    pub gems: Vec<MagicGem>
}

pub struct EchoSystem;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

pub struct Student {
    pub name: String,
    pub grade: u8
}

#[derive(Debug, Clone)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub priority: u8,
    pub status: Status
}

pub struct Time {
    pub hour: u32,
    pub minute: u32
}

#[derive(Debug, PartialEq)]
pub enum Permission {
    Guest,
    User,
    Root
}

pub struct User {
    pub name: String,
    pub permission: Permission,
    pub is_active: bool
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

        Ok(input.trim().to_string().to_lowercase())
    }

    pub fn sleeep(time: u64) {
        std::thread::sleep(std::time::Duration::from_millis(time))
    }
}

impl Adventurer {
    pub fn new(name: &str) -> Self {
        Adventurer { 
            name: name.into(),
            gems: Vec::new()
        }
    }

    pub fn collect_gem(&mut self, gem: MagicGem) {
        println!("collect gem: {}", self.name);
        self.gems.push(gem)
    }

    pub fn report(&self){
        println!("\n--- report adv: {} ---", self.name);
        for gem in &self.gems {
            println!("gem: color: {}, power: {}, rarity: {:?}", gem.color, gem.power, gem.rarity)
        }
    }
}

impl Student {
    pub fn check_student(&self) {
        println!("check about {}", self.name);

        if self.grade < 50 {
            println!("failed get out");
        }

        if self.grade >= 50 && self.grade < 70 {
            println!("Barely Passed");
        }

        if self.grade >= 70 && self.grade < 90 {
            println!("Good");
        }

        if self.grade >= 90 {
            println!("Execllent");
        }
        println!("-------------------")
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

impl Operator {
    pub fn parse_operator(op: &str) -> Option<Self> {
        match op {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            _ => None
        }
    }
}

impl User {
    pub fn new(name: &str, permission: Permission) -> Self {
        Self {
            name: name.to_string(),
            permission,
            is_active: true
        }
    }

    pub fn info(&self) {
        println!("Name: {}, Permission {:?}, is Active {}",
            self.name, self.permission, self.is_active);
    }

    pub fn list_users(users: &[User]) {
        for user in users {
            user.info();
        }
    }
    
    pub fn toggle_active(&mut self) {
        self.is_active = !self.is_active
    }
}
