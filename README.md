#Password Manager

This application is a work-in-progress, designed to securely manage your passwords while offering flexibility and ease of use. Currently, it serves as a basic password manager with essential features and lays the groundwork for more advanced functionality in the future.

---
![pwm](https://github.com/user-attachments/assets/0d8eb127-82ac-4b01-a68e-e49c45a0a551)

## 🚀 Features

### Current Functionality

1. **Add Entries**:
    
    - Store credentials including `username`, `service`, and `password` securely.
    - Each entry is assigned a unique ID automatically.
2. **List Entries**:
    
    - View all stored entries in a simple list format.
    - Passwords can be toggled for visibility.
3. **Edit Entries**:
    
    - Update existing credentials by specifying the entry ID.
4. **Delete Entries**:
    
    - Remove an entry permanently using its ID.
5. **Toggle Password Visibility**:
    
    - Show or hide passwords for individual entries.
6. **Archive Support**:
    
    - Save and load your credentials to/from a secure `.arc` file format.
    - Uses **PBKDF2** for key derivation and **Salsa20** for encryption.
    - Compresses data for efficient storage.

---

## 📖 How It Works

### Password Manager (In-Memory Operations)

The program operates with an in-memory structure for managing credentials:

- **`PasswordManager`**: A structure containing a list of entries and a counter for generating unique IDs.

#### Methods:

- **Add Entry**: Call the `add_entry` method with `username`, `service`, and `password` to add a new record.
    
- **List Entries**: The `list_entries` method retrieves a list of all entries. Passwords are hidden by default.
    
- **Edit Entry**: Update an entry by providing the ID along with the new details.
    
- **Delete Entry**: Remove an entry by its ID using the `delete_entry` method.
    
- **Toggle Password Visibility**: Use the `toggle_show_password` method with an entry ID to show or hide the password for that entry.
    

---

### Archive System (File-Based Storage)

The archive system provides secure file-based storage for credentials.

#### Save Process:

1. Entries are serialized into JSON format.
2. Data is compressed using zlib.
3. A unique salt is generated for key derivation.
4. The passphrase is used with PBKDF2 to derive a 256-bit encryption key.
5. Data is encrypted using Salsa20.
6. The file is saved with a header (magic bytes, version), salt, and encrypted data.

#### Load Process:

1. The `.arc` file is validated using the header.
2. Salt is read to regenerate the encryption key from the passphrase.
3. Data is decrypted and decompressed.
4. Entries are reconstructed from the JSON data.

---

## ⚙️ Installation and Usage

### Prerequisites

- Rust programming language and Cargo installed on your system.
- A terminal or IDE to run the application.

### Steps

1. Clone the repository:
    
    ```bash
    git clone https://github.com/aidan729/Password-Manager.git
    cd Password-Manager
    ```
    
2. Build the project:
    
    ```bash
    cargo build --release
    ```
    
3. Run the application:
    
    ```bash
    cargo run
    ```
    
4. Use the `PasswordManager` API to add, list, edit, or delete entries, or to save/load archives.
    

---

## 🛠️ Planned Features

This project is in its early stages of development. Here’s what’s coming:

- **Enhanced Security**:
    
    - [ ] Password History Tracking: Store a secure history of old passwords to prevent reusing them accidentally.
    - [ ] Breach Monitoring: Notify users if their passwords or services are part of known data breaches using APIs like HaveIBeenPwned.
    - [ ] Customizable Password Policies: Allow users to define minimum password lengths, complexity requirements, and expiration periods.
    - [ ] Idle Lock Mechanism: Automatically lock the app after a period of inactivity.
- **Additional Features**:
    
    - [ ] Batch Import/Export:
        - [ ] Import passwords from other password managers (CSV, JSON).
        - [ ] Export passwords securely for backups or migration.
    - [ ] User Roles and Permissions: Allow shared accounts with controlled access for families or teams.
    - [ ] Multiple Vaults: Create separate vaults for personal, work, or shared accounts.
    - [ ] Color-Coded Categories: Assign categories or tags to entries for better organization.
    - [ ] Light/Dark Mode: User interface theme toggle for better usability.

---

## 🤝 Contributing

Contributions are welcome! If you have ideas or want to help build new features, feel free to open issues or submit pull requests.

---

## 📜 License

This project is licensed under the MIT License.

---

### ⚠️ Disclaimer

This is a **rudimentary password manager** and not yet production-ready. While the current implementation uses strong encryption practices, use it cautiously and avoid storing sensitive credentials until the project matures further.

Thank you for exploring this project! 🚀
