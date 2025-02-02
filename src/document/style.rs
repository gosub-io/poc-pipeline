use std::collections::HashMap;

pub struct StylePropertyList {
    pub properties: HashMap<String, String>,
}

impl StylePropertyList {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn set_property(&mut self, name: &str, value: &str) {
        self.properties.insert(name.to_string(), value.to_string());
    }

    pub fn get_property(&self, name: &str) -> Option<&str> {
        match self.properties.get(name) {
            Some(value) => Some(value.as_str()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_property() {
        let mut style = StylePropertyList::new();
        style.set_property("color", "red");
        assert_eq!(style.get_property("color"), Some("red"));
    }
}