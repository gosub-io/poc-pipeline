use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Percent,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
    Named(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Display {
    Block,
    Inline,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    Number(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StyleValue {
    Keyword(String),
    Unit(f32, Unit),
    Color(Color),
    None,
    Display(Display),
    FontWeight(FontWeight),
}

#[derive(Debug, Clone)]
pub struct StylePropertyList {
    pub properties: HashMap<String, StyleValue>,
}

impl StylePropertyList {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn set_property(&mut self, name: &str, value: StyleValue) {
        self.properties.insert(name.to_string(), value.clone());
    }

    pub fn get_property(&self, name: &str) -> Option<&StyleValue> {
        self.properties.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_property() {
        let mut style = StylePropertyList::new();

        let val = StyleValue::Color(Color::Named("red".to_string()));
        style.set_property("color", val.clone());

        assert_eq!(style.get_property("color"), Some(&val.clone()));
    }
}