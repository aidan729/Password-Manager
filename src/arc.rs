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

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn save(&self, file_path: &str, passphrase: &str) -> io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        let json_data = serde_json::to_string(&self.entries).unwrap();

        let compressed_data = compress_data(json_data.as_bytes());

        let salt = generate_salt();
        let key = derive_key(passphrase, &salt);

        let encrypted_data = encrypt_data(&key, &compressed_data);

        writer.write_all(self.magic.as_bytes())?;
        writer.write_all(self.version.as_bytes())?;

        writer.write_all(&salt)?;

        writer.write_all(&encrypted_data)?;

        writer.flush()?;
        Ok(())
    }

    pub fn load(file_path: &str, passphrase: &str) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let mut reader = BufReader::new(file);

        let mut magic = [0u8; 3];
        let mut version = [0u8; 3];
        reader.read_exact(&mut magic)?;
        reader.read_exact(&mut version)?;

        if magic != *b"arc" || version != *b"1.0" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid .arc file format"));
        }

        let mut salt = [0u8; SALT_SIZE];
        reader.read_exact(&mut salt)?;

        let key = derive_key(passphrase, &salt);

        let mut encrypted_data = Vec::new();
        reader.read_to_end(&mut encrypted_data)?;

        let decrypted_data = decrypt_data(&key, &encrypted_data)?;

        let decompressed_data = decompress_data(&decrypted_data)?;

        let entries: Vec<Entry> = serde_json::from_slice(&decompressed_data)?;

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