#[derive(Debug, Clone)]
pub struct Entry {
    pub id: u32,
    pub username: String,
    pub service: String,
    pub password: String,
    pub show_password: bool,
}

pub struct PasswordManager {
    pub entries: Vec<Entry>,
    pub next_id: u32,
}

impl PasswordManager {
    pub fn new() -> PasswordManager {
        PasswordManager {
            entries: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_entry(&mut self, username: String, service: String, password: String) {
        let entry = Entry {
            id: self.next_id,
            username,
            service,
            password,
            show_password: false,
        };
        self.entries.push(entry);
        self.next_id += 1;
    }

    pub fn list_entries(&self) -> Vec<Entry> {
        self.entries.clone() // Return a cloned Vec of entries
    }

    pub fn toggle_show_password(&mut self, id: u32) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.show_password = !entry.show_password;
        }
    }

    pub fn edit_entry(&mut self, id: u32, username: String, service: String, password: String) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.username = username;
            entry.service = service;
            entry.password = password;
        }
    }

    pub fn delete_entry(&mut self, id: u32) {
        self.entries.retain(|entry| entry.id != id);
    }
}