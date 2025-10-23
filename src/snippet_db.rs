use std::collections::HashMap;

pub struct SnippetDB {
    data: HashMap<String, String>,
}

impl SnippetDB {
    fn new() -> Self {
        SnippetDB {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, name: &str, content: &str) {
        self.data.insert(name.to_string(), content.to_string());
    }

    fn get(&self, name: &str) -> Option<&str> {
        self.data.get(name).map(|s| s.as_str())
    }
}