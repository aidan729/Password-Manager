use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter};
use rand::Rng;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use salsa20::Salsa20;
use cipher::{KeyIvInit, StreamCipher};  // Use cipher crate traits and types
use flate2::{Compression, write::ZlibEncoder, read::ZlibDecoder};

const SALT_SIZE: usize = 16;
const PBKDF2_ITERATIONS: u32 = 100_000;
const KEY_SIZE: usize = 32; // Salsa20 key size is 32 bytes

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub id: u32,
    pub username: String,
    pub service: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Archive {
    magic: String,
    version: String,
    pub entries: Vec<Entry>,
}

impl Archive {
    pub fn new() -> Self {
        Archive {
            magic: "arc".to_string(),
            version: "1.0".to_string(),
            entries: Vec::new(),
        }
    }

    // Add entry to archive
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    // Save archive to an .arc file
    pub fn save(&self, file_path: &str, passphrase: &str) -> io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        // Serialize entries to JSON
        let json_data = serde_json::to_string(&self.entries).unwrap();

        // Compress the JSON data
        let compressed_data = compress_data(json_data.as_bytes());

        // Generate a unique salt and derive the key using PBKDF2
        let salt = generate_salt();
        let key = derive_key(passphrase, &salt);

        // Encrypt the compressed data
        let encrypted_data = encrypt_data(&key, &compressed_data);

        // Write magic header and version
        writer.write_all(self.magic.as_bytes())?;
        writer.write_all(self.version.as_bytes())?;

        // Write the salt to the file (16 bytes)
        writer.write_all(&salt)?;

        // Write the encrypted data to the file
        writer.write_all(&encrypted_data)?;

        writer.flush()?;
        Ok(())
    }

    // Load archive from an .arc file
    pub fn load(file_path: &str, passphrase: &str) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let mut reader = BufReader::new(file);

        // Read the magic header and version
        let mut magic = [0u8; 3];
        let mut version = [0u8; 3];
        reader.read_exact(&mut magic)?;
        reader.read_exact(&mut version)?;

        // Verify magic and version
        if magic != *b"arc" || version != *b"1.0" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid .arc file format"));
        }

        // Read the salt (16 bytes)
        let mut salt = [0u8; SALT_SIZE];
        reader.read_exact(&mut salt)?;

        // Derive the key
        let key = derive_key(passphrase, &salt);

        // Read the rest of the encrypted data
        let mut encrypted_data = Vec::new();
        reader.read_to_end(&mut encrypted_data)?;

        // Decrypt the data
        let decrypted_data = decrypt_data(&key, &encrypted_data)?;

        // Decompress the JSON data
        let decompressed_data = decompress_data(&decrypted_data)?;

        // Deserialize the JSON data
        let entries: Vec<Entry> = serde_json::from_slice(&decompressed_data)?;

        // Return the archive with the loaded entries
        Ok(Archive {
            magic: "arc".to_string(),
            version: "1.0".to_string(),
            entries,
        })
    }
}

fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn decompress_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

fn derive_key(passphrase: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = vec![0u8; KEY_SIZE];
    pbkdf2_hmac::<Sha256>(passphrase.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

fn encrypt_data(key: &[u8], data: &[u8]) -> Vec<u8> {
    let nonce = generate_nonce(); // Salsa20 uses a 64-bit nonce
    let mut cipher = Salsa20::new(key.into(), &nonce.into());
    let mut buffer = data.to_vec();
    cipher.apply_keystream(&mut buffer);
    // Return encrypted data with nonce prepended
    [nonce.to_vec(), buffer].concat()
}

fn decrypt_data(key: &[u8], data: &[u8]) -> io::Result<Vec<u8>> {
    if data.len() < 8 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid encrypted data"));
    }
    let nonce = &data[..8]; // First 8 bytes are the nonce
    let encrypted_data = &data[8..];
    let mut cipher = Salsa20::new(key.into(), nonce.into());
    let mut buffer = encrypted_data.to_vec();
    cipher.apply_keystream(&mut buffer);
    Ok(buffer)
}

fn generate_nonce() -> [u8; 8] {
    let mut nonce = [0u8; 8];
    rand::thread_rng().fill(&mut nonce);
    nonce
}

fn generate_salt() -> [u8; SALT_SIZE] {
    let mut salt = [0u8; SALT_SIZE];
    rand::thread_rng().fill(&mut salt);
    salt
}