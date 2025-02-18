use taffy::{Dimension, Display, FlexDirection, FlexWrap, LengthPercentage, LengthPercentageAuto, Position, Style};
use taffy::prelude::FromLength;
use crate::common::document::style::{StyleProperty, StylePropertyList, StyleValue, Display as CssDisplay, Unit as CssUnit };

pub struct CssTaffyConverter {
    data: StylePropertyList,
}

impl CssTaffyConverter {
    pub fn new(data: &StylePropertyList) -> Self {
        Self {
            data: data.clone(),
        }
    }

    fn get_f32(&self, prop: StyleProperty, default: f32) -> f32 {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match *val {
            StyleValue::Number(num) => num,
            StyleValue::Unit(val, _) => default,
            StyleValue::Keyword(_) => default,
            StyleValue::Color(_) => default,
            StyleValue::None => default,
            StyleValue::Display(_) => default,
            StyleValue::FontWeight(_) => default,
        }
    }

    pub fn convert(&self, ts: &mut Style) {
        ts.display = self.get_display(ts.display);
        // item_is_table: false,
        // box_sizing: BoxSizing::BorderBox,
        // overflow: Point { x: Overflow::Visible, y: Overflow::Visible },
        ts.scrollbar_width = self.get_f32(StyleProperty::ScrollbarWidth, ts.scrollbar_width);
        ts.position = self.get_position(ts.position);
        // inset: Rect::auto(),
        ts.margin.top = self.get_lpa(StyleProperty::MarginTop, ts.margin.top);
        ts.margin.right = self.get_lpa(StyleProperty::MarginRight, ts.margin.right);
        ts.margin.bottom = self.get_lpa(StyleProperty::MarginBottom, ts.margin.bottom);
        ts.margin.left = self.get_lpa(StyleProperty::MarginLeft, ts.margin.left);
        ts.padding.top = self.get_lp(StyleProperty::PaddingTop, ts.padding.top);
        ts.padding.right = self.get_lp(StyleProperty::PaddingRight, ts.padding.right);
        ts.padding.bottom = self.get_lp(StyleProperty::PaddingBottom, ts.padding.bottom);
        ts.padding.left = self.get_lp(StyleProperty::PaddingLeft, ts.padding.left);
        ts.border.top = self.get_lp(StyleProperty::BorderTopWidth, ts.border.top);
        ts.border.right = self.get_lp(StyleProperty::BorderRightWidth, ts.border.right);
        ts.border.bottom = self.get_lp(StyleProperty::BorderBottomWidth, ts.border.bottom);
        ts.border.left = self.get_lp(StyleProperty::BorderLeftWidth, ts.border.left);
        ts.size.width = self.get_dimension(StyleProperty::Width, ts.size.width);
        ts.size.height = self.get_dimension(StyleProperty::Height, ts.size.height);
        ts.min_size.width = self.get_dimension(StyleProperty::MinWidth, ts.min_size.width);
        ts.min_size.height = self.get_dimension(StyleProperty::MinHeight, ts.min_size.height);
        ts.max_size.width = self.get_dimension(StyleProperty::MaxWidth, ts.max_size.width);
        ts.max_size.height = self.get_dimension(StyleProperty::MaxHeight, ts.max_size.height);
        // aspect_ratio: None,
        // gap: Size::zero(),
        // align_items: None,
        // align_self: None,
        // justify_items: None,
        // justify_self: None,
        // align_content: None,
        // justify_content: None,
        // text_align: TextAlign::Auto,
        ts.flex_direction = self.get_flex_direction(ts.flex_direction);
        ts.flex_wrap = self.get_flex_wrap(ts.flex_wrap);
        ts.flex_grow = self.get_f32(StyleProperty::FlexGrow, ts.flex_grow);
        ts.flex_shrink = self.get_f32(StyleProperty::FlexShrink, ts.flex_shrink);
        ts.flex_basis = self.get_flex_basis(ts.flex_basis);
        // grid_template_rows: GridTrackVec::new(),
        // grid_template_columns: GridTrackVec::new(),
        // grid_auto_rows: GridTrackVec::new(),
        // grid_auto_columns: GridTrackVec::new(),
        // grid_auto_flow: GridAutoFlow::Row,
        // grid_row: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
        // grid_column: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
    }

    fn get_flex_wrap(&self, default: FlexWrap) -> FlexWrap {
        let Some(val) = self.data.get_property(StyleProperty::FlexWrap) else {
            return default;
        };

        match *val {
            StyleValue::Keyword(ref val) => {
                match val.as_str() {
                    "nowrap" => FlexWrap::NoWrap,
                    "wrap" => FlexWrap::Wrap,
                    "wrap-reverse" => FlexWrap::WrapReverse,
                    _ => default,
                }
            },
            _ => default,
        }
    }

    fn get_flex_basis(&self, default: Dimension) -> Dimension {
        let Some(val) = self.data.get_property(StyleProperty::FlexBasis) else {
            return default;
        };

        match val {
            StyleValue::Unit(val, unit) => Dimension::from_length(*val),
            StyleValue::Number(val) => Dimension::from_length(*val),
            StyleValue::Keyword(val) if val == "auto" => Dimension::Auto,
            _ => default,
        }
    }

    fn get_flex_direction(&self, default: FlexDirection) -> FlexDirection {
        let Some(val) = self.data.get_property(StyleProperty::FlexDirection) else {
            return default;
        };

        match *val {
            StyleValue::Keyword(ref val) => {
                match val.as_str() {
                    "row" => FlexDirection::Row,
                    "row-reverse" => FlexDirection::RowReverse,
                    "column" => FlexDirection::Column,
                    "column-reverse" => FlexDirection::ColumnReverse,
                    _ => default,
                }
            },
            _ => default,
        }
    }

    fn get_display(&self, default: Display) -> Display {
        let Some(val) = self.data.get_property(StyleProperty::Display) else {
            return default;
        };

        match val {
            StyleValue::Display(val) => {
                match val {
                    CssDisplay::Block => Display::Block,
                    CssDisplay::Flex => Display::Flex,
                    CssDisplay::None => Display::None,
                    CssDisplay::Inline => Display::Flex,
                }
            }
            _ => default,
        }
    }

    fn get_position(&self, default: Position) -> Position {
        let Some(val) = self.data.get_property(StyleProperty::Position) else {
            return default;
        };

        match val {
            StyleValue::Keyword(ref val) => {
                match val.as_str() {
                    "relative" => Position::Relative,
                    "absolute" => Position::Absolute,
                    _ => default,
                }
            },
            _ => default,
        }
    }

    fn get_lpa(&self, prop: StyleProperty, default: LengthPercentageAuto) -> LengthPercentageAuto {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match val {
            StyleValue::Unit(value, unit) => {
                match unit {
                    CssUnit::Px => LengthPercentageAuto::Length(*value),
                    CssUnit::Percent => LengthPercentageAuto::Percent(*value),
                    _ => default,
                }
            }
            StyleValue::Number(value) => LengthPercentageAuto::Length(*value),
            StyleValue::Keyword(val) if val == "auto" => LengthPercentageAuto::Auto,
            _ => default,
        }
    }

    fn get_lp(&self, prop: StyleProperty, default: LengthPercentage) -> LengthPercentage {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match val {
            StyleValue::Unit(value, unit) => {
                match unit {
                    CssUnit::Px => LengthPercentage::Length(*value),
                    CssUnit::Percent => LengthPercentage::Percent(*value),
                    _ => default,
                }
            }
            StyleValue::Number(value) => LengthPercentage::Length(*value),
            _ => default,
        }
    }

    fn get_dimension(&self, prop: StyleProperty, default: Dimension) -> Dimension {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match val {
            StyleValue::Unit(value, unit) => {
                match unit {
                    CssUnit::Px => Dimension::from_length(*value),
                    CssUnit::Percent => Dimension::from_length(*value),
                    _ => default,
                }
            }
            StyleValue::Number(value) => Dimension::from_length(*value),
            _ => default,
        }
    }
}