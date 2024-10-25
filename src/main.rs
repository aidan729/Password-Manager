use eframe::egui::{self, CentralPanel, Context};
pub mod util;
pub mod arc;
use crate::util::PasswordManager;
use crate::arc::Archive;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use std::env;

pub struct AppState {
    username_input: String,
    service_input: String,
    password_input: String,
    password_manager: Arc<Mutex<PasswordManager>>,
    file_path: PathBuf,
    passphrase: String,
}

impl AppState {
    fn new() -> Self {
        // Get the username from the environment and construct the file path
        let user = env::var("USER").unwrap_or("default_user".to_string());
        let file_path = PathBuf::from(format!("C:/Users/{}/arc/passwords.arc", user));
        let passphrase = user.clone(); // Use the username as the passphrase for now
        
        // Create the directory if it doesn't exist
        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir).unwrap();
        }

        // Check if the file exists, if not create a new one with empty data
        if !file_path.exists() {
            let mut archive = Archive::new();
            archive.save(file_path.to_str().unwrap(), &passphrase).unwrap();
        }

        // Load existing data
        let password_manager = Arc::new(Mutex::new(PasswordManager::new()));
        if let Ok(archive) = Archive::load(file_path.to_str().unwrap(), &passphrase) {
            for entry in archive.entries {
                password_manager.lock().unwrap().add_entry(
                    entry.username, entry.service, entry.password
                );
            }
        }

        Self {
            username_input: String::new(),
            service_input: String::new(),
            password_input: String::new(),
            password_manager,
            file_path,
            passphrase,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Password Manager");
            
            // Input fields
            ui.horizontal(|ui| {
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.username_input);
            });
            ui.horizontal(|ui| {
                ui.label("Service:");
                ui.text_edit_singleline(&mut self.service_input);
            });
            ui.horizontal(|ui| {
                ui.label("Password:");
                ui.text_edit_singleline(&mut self.password_input);
            });

            // Add entry button
            if ui.button("Add Entry").clicked() {
                let username = self.username_input.clone();
                let service = self.service_input.clone();
                let password = self.password_input.clone();

                // Add the entry to the in-memory password manager
                let mut manager = self.password_manager.lock().unwrap();
                manager.add_entry(username.clone(), service.clone(), password.clone());

                // Save the updated password data to the .arc file
                let mut archive = Archive::new();
                for entry in manager.list_entries() {
                    archive.add_entry(arc::Entry {
                        id: entry.id,
                        username: entry.username.clone(),
                        service: entry.service.clone(),
                        password: entry.password.clone(),
                    });
                }
                archive.save(self.file_path.to_str().unwrap(), &self.passphrase).unwrap();

                // Clear input fields after adding
                self.username_input.clear();
                self.service_input.clear();
                self.password_input.clear();
            }

            // Separator for the list of stored entries
            ui.separator();
            ui.heading("Stored Entries:");

            let entries = {
                let manager = self.password_manager.lock().unwrap();
                manager.list_entries()
            };

            // Display each entry
            for entry in entries {
                ui.horizontal(|ui| {
                    ui.label(format!("ID: {}", entry.id));
                    ui.label(format!("Username: {}", entry.username));
                    ui.label(format!("Service: {}", entry.service));

                    if entry.show_password {
                        ui.label(format!("Password: {}", entry.password));
                    }

                    if ui.button(if entry.show_password { "Hide Password" } else { "Show Password" }).clicked() {
                        let mut manager = self.password_manager.lock().unwrap();
                        manager.toggle_show_password(entry.id);
                    }
                });
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Password Manager",
        options,
        Box::new(|_cc| Box::new(AppState::new())),
    );
}