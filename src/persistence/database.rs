use std::collections::HashMap;

pub trait DatabaseAlg: Send + Sync {
    fn store(&mut self, long_url: String, short_url: String) -> ();
    fn get_all(&self) -> HashMap<String, String>;
    fn get_long_url_with_short_url(&self, short_url: String) -> Option<String>;
}

pub struct InMemoryDatabase {
    store: HashMap<String, String>,
}

impl DatabaseAlg for InMemoryDatabase {
    fn store(&mut self, long_url: String, short_url: String) -> () {
        self.store.insert(long_url, short_url);
    }

    fn get_all(&self) -> HashMap<String, String> {
        self.store.clone()
    }
    
    fn get_long_url_with_short_url(&self, short_url: String) -> Option<String> {
        self.store.iter()
            .find_map(|(key, val)| 
                if val == &short_url { Some(key.clone()) } else { None }
            )
    }
}

impl InMemoryDatabase {
    // like a companion object
    pub fn new(in_memory_store: HashMap<String, String>) -> Self {
        InMemoryDatabase {
            store: in_memory_store,
        }
    }
}
