use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum StyleProperty {
    Color,
    BackgroundColor,
    FontSize,
    FontWeight,
    Display,
    Width,
    Height,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    BorderBottomWidth,
    BorderTopWidth,
    BorderLeftWidth,
    BorderRightWidth,
    MarginBlockStart,
    MarginBlockEnd,
    FontFamily,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Percent,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
    Named(String),
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum Display {
    Block,
    Inline,
    None,
}

#[allow(unused)]
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
    pub properties: HashMap<StyleProperty, StyleValue>,
}

impl StylePropertyList {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn set_property(&mut self, prop: StyleProperty, value: StyleValue) {
        self.properties.insert(prop, value.clone());
    }

    pub fn get_property(&self, prop: StyleProperty) -> Option<&StyleValue> {
        self.properties.get(&prop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_property() {
        let mut style = StylePropertyList::new();

        let val = StyleValue::Color(Color::Named("red".to_string()));
        style.set_property(StyleProperty::Color, val.clone());

        assert_eq!(style.get_property(StyleProperty::Color), Some(&val.clone()));
    }
}