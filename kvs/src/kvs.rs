use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, SeekFrom, Seek};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsError, Result};

/// Doc
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    /// Doc
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }
    /// Doc
    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

/// The KvStore stores key/value pairs in memory.
///
/// Example
/// ```rust
/// use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("foo".to_owned(), "bar".to_owned());
/// let value = store.get("foo".to_owned());
/// assert_eq!(value, Some("bar".to_owned()));
/// ```
pub struct KvStore {
    map: HashMap<String, String>,
    file_path: PathBuf,
    writer: File,
}

impl KvStore {
    /// Doc
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = &path.into();
        let file_path = &path.join("kvs.json");
        dbg!(&file_path);
        fs::create_dir_all(path)?;
        let writer = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
        let mut store = KvStore {
            map: HashMap::new(),
            file_path: file_path.clone(),
            writer,
        };
        store.index_from_log()?;
        Ok(store)
    }

    /// Sets the sting value of a string key.
    ///
    /// If the key already exists, the value is overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::set(key, value);
        self.write_to_memory(&command)?;
        self.write_to_log(&command)?;
        Ok(())
    }

    /// Gets the value of a given string key.
    ///
    /// Returns `None` if the key doesn't exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.index_from_log()?;
        self.read_from_memory(key)
    }

    /// Remove the key/value pair for the given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if let None = self.get(key.clone())? {
            return Err(KvsError::KeyNotFound());
        }
        let command = Command::remove(key);
        self.write_to_log(&command)?;
        Ok(())
    }

    /// Doc
    fn write_to_memory(&mut self, command: &Command) -> Result<()> {
        match command {
            Command::Set {key, value} => {self.map.insert(key.clone(), value.clone());},
            Command::Remove {key} => {self.map.remove(key);},
        }
        Ok(())
    }

    /// Doc
    fn read_from_memory(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    /// Doc
    fn write_to_log(&mut self, command: &Command) -> Result<()> {
        let serialized = serde_json::to_string(command)?;
        self.writer.seek(SeekFrom::End(0))?;
        self.writer.write_all(serialized.as_bytes())?;
        Ok(())
    }

    /// Doc
    fn index_from_log(&mut self) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let stream = Deserializer::from_str(&data).into_iter::<Command>();
        for cmd in stream {
            match cmd {
                Ok(cmd) => {
                    self.write_to_memory(&cmd)?;
                }
                Err(err) => return Err(KvsError::from(err)),
            }
        }
        Ok(())
    }
}
