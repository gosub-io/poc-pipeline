use taffy::{AlignContent, AlignItems, AlignSelf, BoxSizing, Dimension, Display, FlexDirection, FlexWrap, LengthPercentage, LengthPercentageAuto, Overflow, Point, Position, Rect, Size, Style, TextAlign};
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

    fn get_f32_opt(&self, prop: StyleProperty, default: Option<f32>) -> Option<f32> {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match *val {
            StyleValue::Number(num) => Some(num),
            _ => default,
        }
    }

    pub fn convert(&self, ts: &mut Style) {
        ts.display = self.get_display(ts.display);
        // item_is_table: false,
        ts.box_sizing = self.get_box_sizing(ts.box_sizing);
        ts.overflow = Point {
            x: self.get_overflow(StyleProperty::OverflowX, ts.overflow.x),
            y: self.get_overflow(StyleProperty::OverflowY, ts.overflow.y),
        };
        ts.scrollbar_width = self.get_f32(StyleProperty::ScrollbarWidth, ts.scrollbar_width);
        ts.position = self.get_position(ts.position);
        ts.inset = self.get_inset(ts.inset);
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
        ts.aspect_ratio = self.get_f32_opt(StyleProperty::AspectRatio, ts.aspect_ratio);
        ts.gap = self.get_size_lp(StyleProperty::Gap, ts.gap);
        ts.align_items = self.get_align_items(StyleProperty::AlignItems, ts.align_items);
        ts.align_self = self.get_align_self(StyleProperty::AlignSelf, ts.align_self);
        ts.justify_items = self.get_align_items(StyleProperty::JustifyItems, ts.justify_items);
        ts.justify_self = self.get_align_self(StyleProperty::JustifySelf, ts.justify_self);
        ts.align_content = self.get_align_content(StyleProperty::AlignContent, ts.align_content);
        ts.justify_content = self.get_align_content(StyleProperty::JustifyContent, ts.justify_content);
        ts.text_align = self.get_text_align(ts.text_align);
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
            StyleValue::Unit(val, _unit) => Dimension::from_length(*val),
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

    fn get_size_lp(&self, prop: StyleProperty, default: Size<LengthPercentage>) -> Size<LengthPercentage> {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match val {
            StyleValue::Unit(value, unit) => {
                match unit {
                    CssUnit::Px => Size::length(*value),
                    CssUnit::Percent => Size::percent(*value),
                    _ => default,
                }
            }
            StyleValue::Number(value) => Size::length(*value),
            _ => default,
        }
    }

    fn get_align_items(&self, prop: StyleProperty, default: Option<AlignItems>) -> Option<AlignItems> {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match val {
            StyleValue::Keyword(ref val) => {
                match val.as_str() {
                    "start" => Some(AlignItems::Start),
                    "end" => Some(AlignItems::End),
                    "flex-start" => Some(AlignItems::FlexStart),
                    "flex-end" => Some(AlignItems::FlexEnd),
                    "center" => Some(AlignItems::Center),
                    "baseline" => Some(AlignItems::Baseline),
                    "stretch" => Some(AlignItems::Stretch),
                    _ => default,
                }
            },
            _ => default,
        }
    }

    fn get_align_self(&self, prop: StyleProperty, default: Option<AlignSelf>) -> Option<AlignSelf> {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match val {
            StyleValue::Keyword(ref val) => {
                match val.as_str() {
                    "auto" => None,
                    "start" => Some(AlignSelf::Start),
                    "end" => Some(AlignSelf::End),
                    "flex-start" => Some(AlignSelf::FlexStart),
                    "flex-end" => Some(AlignSelf::FlexEnd),
                    "center" => Some(AlignSelf::Center),
                    "baseline" => Some(AlignSelf::Baseline),
                    "stretch" => Some(AlignSelf::Stretch),
                    _ => default,
                }
            },
            _ => default,
        }
    }

    fn get_align_content(&self, prop: StyleProperty, default: Option<AlignContent>) -> Option<AlignContent> {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match val {
            StyleValue::Keyword(ref val) => {
                dbg!(&val);
                match val.as_str() {
                    "start" => Some(AlignContent::Start),
                    "end" => Some(AlignContent::End),
                    "flex-start" => Some(AlignContent::FlexStart),
                    "flex-end" => Some(AlignContent::FlexEnd),
                    "center" => Some(AlignContent::Center),
                    "stretch" => Some(AlignContent::Stretch),
                    "space-between" => Some(AlignContent::SpaceBetween),
                    "space-evenly" => Some(AlignContent::SpaceEvenly),
                    "space-around" => Some(AlignContent::SpaceAround),
                    _ => default,
                }
            },
            _ => default,
        }
    }

    fn get_text_align(&self, default: TextAlign) -> TextAlign {
        let Some(val) = self.data.get_property(StyleProperty::TextAlign) else {
            return default;
        };

        match val {
            StyleValue::Keyword(ref val) => {
                match val.as_str() {
                    "auto" => TextAlign::Auto,
                    "center" => TextAlign::LegacyCenter,
                    "left" => TextAlign::LegacyLeft,
                    "right" => TextAlign::LegacyRight,
                    _ => default,
                }
            },
            _ => default,
        }
    }

    fn get_inset(&self, default: Rect<LengthPercentageAuto>) -> Rect<LengthPercentageAuto> {
        Rect {
            top: self.get_lpa(StyleProperty::InsetBlockStart, default.top),
            right: self.get_lpa(StyleProperty::InsetInlineEnd, default.right),
            bottom: self.get_lpa(StyleProperty::InsetBlockEnd, default.bottom),
            left: self.get_lpa(StyleProperty::InsetInlineStart, default.left),
        }
    }

    fn get_overflow(&self, prop: StyleProperty, default: Overflow) -> Overflow {
        let Some(val) = self.data.get_property(prop) else {
            return default;
        };

        match val {
            StyleValue::Keyword(ref val) => {
                match val.as_str() {
                    "visible" => Overflow::Visible,
                    "hidden" => Overflow::Hidden,
                    "scroll" => Overflow::Scroll,
                    "clip" => Overflow::Clip,
                    _ => default,
                }
            },
            _ => default,
        }
    }

    fn get_box_sizing(&self, default: BoxSizing) -> BoxSizing {
        let Some(val) = self.data.get_property(StyleProperty::BoxSizing) else {
            return default;
        };

        match val {
            StyleValue::Keyword(ref val) => {
                match val.as_str() {
                    "content-box" => BoxSizing::ContentBox,
                    "border-box" => BoxSizing::BorderBox,
                    _ => default,
                }
            },
            _ => default,
        }
    }
}