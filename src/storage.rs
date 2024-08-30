use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::time::{Duration, Instant};
use crate::errors::app_errors::AppError;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::constants::{DEFAULT_CHANGE_THRESHOLD, DEFAULT_SNAPSHOT_PERIOD};

#[derive(Debug)]
pub struct Item {
    pub value: String,
    pub created: Instant,
    pub expires: usize,
}

#[derive(Debug)]
pub struct Snapshot {
    change_count: u32,
    snapshot_change_threshold: u32,
    pub snapshot_period_secs: u32,
    last_snapshot_time: Instant
}

#[derive(Debug)]
pub struct Storage {
    pub items: HashMap<String, Item>,
    dump_path: String,
    pub snapshot: Snapshot
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            items: HashMap::new(),
            dump_path: String::from("src/dump/dump.rdb"),
            snapshot: Snapshot {
                change_count: 0,
                snapshot_change_threshold: DEFAULT_CHANGE_THRESHOLD,
                snapshot_period_secs: DEFAULT_SNAPSHOT_PERIOD,
                last_snapshot_time: Instant::now(),
            }
        }
    }

    pub fn set(&mut self, key: String, value: String, expires: usize) {
        let item = Item {
            value,
            created: Instant::now(),
            expires,
        };

        self.items.insert(key, item);
        self.snapshot.change_count += 1;
    }

    pub fn get(&mut self, key: &str) -> Option<&Item> {
        let item = self.items.get(key)?;
        let is_expired = item.expires > 0 && item.created.elapsed().as_millis() > item.expires as u128;

        match is_expired {
            true => None,
            false => Some(item),
        }
    }

    pub fn del(&mut self, keys: Vec<&str>) -> u16 {
        let mut deleted_items = 0;
        for key in keys {
            if let Some(_) = self.items.remove(key) {
                deleted_items += 1;
                self.snapshot.change_count += 1;
            }
        }
        deleted_items
    }

    pub fn keys(&mut self, expr: &str) -> Result<Vec<String>, AppError> {
        if expr.contains('*') {
            let pattern = expr.replace("*", ".*");
            let regex = regex::Regex::new(&pattern).map_err(|_| AppError::InvalidPattern)?;

            let keys = self.items.keys()
                .filter(|key| regex.is_match(key))
                .cloned()
                .collect();
            return Ok(keys);
        }
        let keys = self.items.keys()
            .filter(|key| key == &expr)
            .cloned()
            .collect();
        Ok(keys)
    }

    pub fn load_rdb_file(&mut self) -> Result<(), AppError> {
        let file = File::open(&self.dump_path).map_err(|e| AppError::FileError(e))?;
        let mut reader = BufReader::new(file);

        let mut header = [0; 5];
        reader.read_exact(&mut header).map_err(|e| AppError::FileError(e))?;
        if &header != b"REDIS" {
            return Err(AppError::InvalidFileFormat);
        }

        let mut version = [0; 4];
        reader.read_exact(&mut version).map_err(AppError::FileError)?;
        if &version != b"0006" {
            return Err(AppError::InvalidFileFormat);
        }

        loop {
            if reader.fill_buf().map_err(AppError::FileError)?.len() < 8 {
                break;
            }

            let created_secs = reader.read_u64::<BigEndian>().map_err(AppError::FileError)?;
            let created = Instant::now() - Duration::from_secs(created_secs);
            let expires = reader.read_u32::<BigEndian>().map_err(AppError::FileError)? as usize;

            let mut key = Vec::new();
            reader.read_until(0, &mut key).map_err(AppError::FileError)?;
            key.pop();
            let key = String::from_utf8(key).map_err(|_| AppError::InvalidFileFormat)?;

            let mut value = Vec::new();
            reader.read_until(0, &mut value).map_err(AppError::FileError)?;
            value.pop();
            let value = String::from_utf8(value).map_err(|_| AppError::InvalidFileFormat)?;

            self.items.insert(key, Item { value, created, expires });
        }

        let mut eof_marker = [0; 3];
        reader.read_exact(&mut eof_marker).map_err(AppError::FileError)?;
        if &eof_marker != b"EOF" {
            return Err(AppError::InvalidFileFormat);
        }

        Ok(())
    }

    pub fn save_rdb_file(&mut self) -> Result<(), AppError> {
        let file = File::create(&self.dump_path).map_err(|e| AppError::FileError(e))?;
        let mut writer = BufWriter::new(file);

        // Header
        writer.write_all(b"REDIS").map_err(|e| AppError::FileError(e))?;
        writer.write_all(b"0006").map_err(|e| AppError::FileError(e))?;

        for (key, item) in &self.items {
            // Created -> Expires -> Key -> Value
            writer.write_u64::<BigEndian>(item.created.elapsed().as_secs()).map_err(|e| AppError::FileError(e))?;

            writer.write_u32::<BigEndian>(item.expires as u32).map_err(|e| AppError::FileError(e))?;

            writer.write_all(key.as_bytes()).map_err(|e| AppError::FileError(e))?;
            writer.write_all(&[0]).map_err(|e| AppError::FileError(e))?;

            writer.write_all(item.value.as_bytes()).map_err(|e| AppError::FileError(e))?;
            writer.write_all(&[0]).map_err(|e| AppError::FileError(e))?;
        }

        // End of file
        writer.write_all(b"EOF").map_err(AppError::FileError)?;

        Ok(())
    }

    pub fn should_take_snapshot(&mut self) -> bool {
        if self.snapshot.change_count > self.snapshot.snapshot_change_threshold &&
            self.snapshot.last_snapshot_time.elapsed() >= Duration::from_secs(self.snapshot.snapshot_period_secs as u64) {
            self.snapshot.change_count = 0;
            return true;
        }
        false
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage::new()
    }
}