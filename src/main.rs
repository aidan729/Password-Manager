use std::io::{self, Write};

#[derive(Debug)]
struct Entry {
    id: u32,
    username: String,
    service: String,
    password: String,
}

struct PasswordManager {
    entries: Vec<Entry>,
    next_id: u32,
}

impl PasswordManager {
    fn new() -> PasswordManager {
        PasswordManager {
            entries: Vec::new(),
            next_id: 1,
        }
    }

    fn add_entry(&mut self, username: String, service: String, password: String) {
        let entry = Entry {
            id: self.next_id,
            username,
            service,
            password,
        };
        self.entries.push(entry);
        self.next_id += 1;
        println!("Entry added with ID {}", self.next_id - 1);
    }

    fn list_entries(&self) {
        println!("ID\tUsername\tService");
        for entry in &self.entries {
            println!("{}\t{}\t{}", entry.id, entry.username, entry.service);
        }
    }

    fn get_password_by_id(&self, id: u32) -> Option<&str> {
        for entry in &self.entries {
            if entry.id == id {
                return Some(&entry.password);
            }
        }
        None
    }
}

fn main() {
    let mut manager = PasswordManager::new();

    loop {
        println!("1. Add Entry\n2. List Entries\n3. Get Password by ID\n4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                let mut username = String::new();
                let mut service = String::new();
                let mut password = String::new();

                print!("Enter username: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut username).unwrap();

                print!("Enter service: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut service).unwrap();

                print!("Enter password: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut password).unwrap();

                manager.add_entry(username.trim().to_string(), service.trim().to_string(), password.trim().to_string());
            }
            "2" => {
                manager.list_entries();
            }
            "3" => {
                let mut id_input = String::new();
                print!("Enter entry ID: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_input).unwrap();

                if let Ok(id) = id_input.trim().parse::<u32>() {
                    match manager.get_password_by_id(id) {
                        Some(password) => println!("Password: {}", password),
                        None => println!("Entry not found."),
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}