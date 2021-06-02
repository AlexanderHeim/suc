//! Simple filebased saving and hashing of user credentials
//! 
//! Provides one struct: SucFile

use std::{fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom, Write}, path::{Path, PathBuf}, str};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use rand::rngs::OsRng;


pub mod session;

/// A SucFile. Represents the actual file.
pub struct SucFile {
    file: File,
    buf: Vec<u8>,
    path: PathBuf,
}

impl SucFile {
    /// Opens an actual file as a sucfile. If the file at the given path does not exist, the file is created and then opened.
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true).open(path)?;
        Ok(SucFile {
            file,
            buf: Vec::new(),
            path: path.to_path_buf(),
        })
    }

    fn gen(key: &str, secret: &str) -> Vec<u8> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password_simple(secret.as_bytes(), salt.as_ref()).unwrap().to_string();

        let mut buf: Vec<u8> = Vec::new();
        buf.push(key.len() as u8);
        buf.extend_from_slice(key.as_bytes());
        buf.push(hash.len() as u8);
        buf.extend_from_slice(hash.as_bytes());
        buf
    }

    /// Add key and hash of value to the file (key would be username, value would be password)
    ///
    /// Returns Ok(()) if successful
    pub fn add(&mut self, key: &str, value: &str) -> std::io::Result<()> {
        if key.len() > 255 || value.len() > 255 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Key and value length have to be less than 256 bytes each!"));
        }
        let tmp = self.get(key)?;
        if tmp.is_some() {
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Key already exists!"));
        }
        let old_cursor = self.file.seek(SeekFrom::Current(0))?;
        self.file.write_all(&SucFile::gen(key, value))?;
        self.file.seek(SeekFrom::Start(old_cursor))?;
        Ok(())
    }

    /// Compares key and hash of value to saved key and saved hash of value.
    ///
    /// Returns Ok(true) if value is correct and Ok(false) if value is incorrect.
    pub fn check(&mut self, key: &str, value: &str) -> std::io::Result<bool> {
        if key.len() > 255 || value.len() > 255 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Key and value length have to be less than 256 bytes each!"));
        }
        let tmp = self.get(key)?;
        if tmp.is_none() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Key is not registered!"));
        }
        let hash_string = tmp.unwrap();
        let argon2 = Argon2::default();
        let hash = PasswordHash::new(&hash_string).unwrap();
        let result = match argon2.verify_password(value.as_bytes(), &hash) {
            Ok(_) => true,
            Err(e) => match e {
                argon2::password_hash::Error::Password => false,
                _ => { panic!("Something went wrong with checking the password hash: {}", e)}
            }
        };
        Ok(result)
    }

    /// Removes a saved key and it's corresponding hashed value from the file.
    ///
    /// Returns Ok(()) if delete was successful.
    pub fn remove(&mut self, key: &str) -> std::io::Result<()> {
        if key.len() > 255 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Key length has to be less than 256 bytes!"));
        }
        let old_cursor = self.file.seek(SeekFrom::Current(0))?;
        self.file.seek(SeekFrom::Start(0))?;
        self.buf.clear();
        self.file.read_to_end(&mut self.buf)?;
        let mut index: usize = 0;
        let mut k_index: Option<usize> = None;
        let mut size: usize = 0;
        while index < self.file.metadata().unwrap().len() as usize {
            let k_len = self.buf[index];
            let s_len = self.buf[index + 1 + k_len as usize];
            let k = str::from_utf8(&self.buf[(index+1)..(index+1+k_len as usize)]).unwrap();
            if k == key {
                k_index = Some(index);
                size = 2 + k_len as usize + s_len as usize;
                break;
            }
            index += 2 + k_len as usize + s_len as usize;
        }
        if k_index.is_none() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Key to remove not found!"));
        }
        let ki = k_index.unwrap();
        for _ in ki..(ki+size) {
            self.buf.remove(ki);
        }
        std::fs::write(self.path.as_path(), "")?;
        self.file.write_all(&self.buf)?;
        self.file.seek(SeekFrom::Start(old_cursor))?;
        Ok(())
    }

    fn get(&mut self, key: &str) -> std::io::Result<Option<String>> {
        self.file.seek(SeekFrom::Start(0))?;
        self.buf.clear();
        self.file.read_to_end(&mut self.buf)?;
        let mut index: usize = 0;
        while index < self.file.metadata().unwrap().len() as usize {
            let k_len = self.buf[index];
            let s_len = self.buf[index + 1 + k_len as usize];
            let k = str::from_utf8(&self.buf[(index+1)..(index+1+k_len as usize)]).unwrap();
            let s = str::from_utf8(&self.buf[(index+2+k_len as usize)..(index+2+k_len as usize + s_len as usize)]).unwrap();
            if k == key {
                return Ok(Some(String::from(s)));
            }
            index += 2 + k_len as usize + s_len as usize;
        }
        Ok(None)
    }
}