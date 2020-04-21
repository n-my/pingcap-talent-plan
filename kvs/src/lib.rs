#![deny(missing_docs)]
//! A simple key/value store.
use std::collections::HashMap;

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
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }
    /// Gets the value of a given string key.
    ///
    /// Returns `None` if the key doesn't exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Sets the sting value of a string key.
    ///
    /// If the key already exists, the value is overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Remove the key/value pair for the given key.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
