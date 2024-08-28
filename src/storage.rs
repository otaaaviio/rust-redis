use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
pub struct Item {
    pub value: String,
    pub created: Instant,
    pub expires: usize,
}

#[derive(Debug)]
pub struct Storage {
    pub items: HashMap<String, Item>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            items: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, expires: usize) {
        let item = Item {
            value,
            created: Instant::now(),
            expires,
        };

        self.items.insert(key, item);
    }

    pub fn get(&mut self, key: &str) -> Option<&Item> {
        let item = self.items.get(key)?;
        let is_expired = item.expires > 0 && item.created.elapsed().as_millis() > item.expires as u128;

        match is_expired {
            true => None,
            false => Some(item),
        }
    }

    pub fn del(&mut self, keys: Vec<&str>) -> i32 {
        let mut deleted_items = 0;
        for key in keys {
            if let Some(_) = self.items.remove(key) {
                deleted_items += 1;
            }
        }
        deleted_items
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage::new()
    }
}