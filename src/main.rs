use eframe::egui::{CentralPanel, Context, RichText, ScrollArea};
pub mod util;
pub mod arc;
use crate::util::PasswordManager;
use crate::arc::Archive;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;
use std::env;
use copypasta::{ClipboardContext, ClipboardProvider};

pub struct AppState {
    username_input: String,
    service_input: String,
    password_input: String,
    password_manager: Arc<Mutex<PasswordManager>>,
    file_path: PathBuf,
    passphrase: String,
    editing_entry_id: Option<u32>,
}

impl AppState {
    fn new() -> Self {
        let user_dir = env::var("USERPROFILE").unwrap_or_else(|_| "C:/Users/default_user".to_string());
        let file_path = PathBuf::from(format!("{}/arc/passwords.arc", user_dir));
        let passphrase = user_dir.clone();

        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir).expect("Failed to create directory for storing passwords.");
        }

        if !file_path.exists() {
            let archive = Archive::new();
            if let Err(e) = archive.save(file_path.to_str().unwrap(), &passphrase) {
                eprintln!("Failed to create initial .arc file: {:?}", e);
            }
        }

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
            editing_entry_id: None,
        }
    }

    fn save_entries(&self) {
        let mut archive = Archive::new();

        {
            let manager = self.password_manager.lock().unwrap();
            for entry in manager.list_entries() {
                archive.add_entry(arc::Entry {
                    id: entry.id,
                    username: entry.username.clone(),
                    service: entry.service.clone(),
                    password: entry.password.clone(),
                });
            }
        }

        archive.save(self.file_path.to_str().unwrap(), &self.passphrase).unwrap();
    }
}


impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Password Manager");

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

            if ui.button(if self.editing_entry_id.is_some() { "Save Changes" } else { "Add Entry" }).clicked() {
                let username = self.username_input.clone();
                let service = self.service_input.clone();
                let password = self.password_input.clone();

                {
                    let mut manager = self.password_manager.lock().unwrap();
                    if let Some(id) = self.editing_entry_id {
                        manager.edit_entry(id, username, service, password);
                        self.editing_entry_id = None;
                    } else {
                        manager.add_entry(username, service, password);
                    }
                }

                self.save_entries();
                self.username_input.clear();
                self.service_input.clear();
                self.password_input.clear();
            }

            ui.separator();
            ui.heading("Stored Entries:");

            ScrollArea::vertical().show(ui, |ui| {
                let entries = {
                    let manager = self.password_manager.lock().unwrap();
                    manager.list_entries()
                };

                for entry in entries {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Username: {}", entry.username)).strong());
                        ui.label(RichText::new(format!("Service: {}", entry.service)).strong());

                        if entry.show_password {
                            ui.label(RichText::new(format!("Password: {}", entry.password)).strong());
                        }

                        if ui.button(if entry.show_password { "Hide Password" } else { "Show Password" }).clicked() {
                            let mut manager = self.password_manager.lock().unwrap();
                            manager.toggle_show_password(entry.id);
                        }

                        if ui.button("Copy to Clipboard").clicked() {
                            if entry.show_password {
                                let mut clipboard = ClipboardContext::new().unwrap();
                                clipboard.set_contents(entry.password.clone()).unwrap();
                            }
                        }

                        if ui.button("Edit").clicked() {
                            self.username_input = entry.username.clone();
                            self.service_input = entry.service.clone();
                            self.password_input.clone();
                            self.editing_entry_id = Some(entry.id);
                        }

                        if ui.button("Delete").clicked() {
                            {
                                let mut manager = self.password_manager.lock().unwrap();
                                manager.delete_entry(entry.id);
                            }
                            self.save_entries();
                        }
                    });
                }
            });
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Password Manager",
        options,
        Box::new(|_cc| Box::new(AppState::new())),
    );
}