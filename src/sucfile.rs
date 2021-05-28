use std::{fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom, Write}, path::Path, str};

pub struct SucFile {
    file: File,
    buf: Vec<u8>,
}

impl SucFile {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true).open(path)?;
        Ok(SucFile {
            file,
            buf: Vec::new(),
        })
    }

    fn to_bytes(key: &str, secret: &str) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(key.len() as u8);
        bytes.extend_from_slice(key.as_bytes());
        bytes.push(secret.len() as u8);
        bytes.extend_from_slice(secret.as_bytes());
        bytes
    }

    pub fn add(&mut self, key: &str, secret: &str) -> std::io::Result<()> {
        if key.len() > 255 || secret.len() > 255 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Key and secret length have to be less than 256 bytes each!"));
        }
        let tmp = self.get(key)?;
        if tmp.is_some() {
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Key already exists!"));
        }
        let old_cursor = self.file.seek(SeekFrom::Current(0))?;
        self.file.write_all(&SucFile::to_bytes(key, secret))?;
        self.file.seek(SeekFrom::Start(old_cursor))?;
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> std::io::Result<Option<String>> {
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