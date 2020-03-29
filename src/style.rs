use iced::{checkbox, Color, Background};

use std::default::Default;

#[derive(Clone, Copy)]
pub struct CheckboxColor {
    pub color: Color,
}

impl checkbox::StyleSheet for CheckboxColor {
    fn active(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color::from_rgb(0.0, 1.0, 0.0)),
            checkmark_color: Color::from_rgb(1.0, 0.0, 0.0),
            border_radius: 9,
            border_width: 2,
            border_color: Color::BLACK,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color::from_rgb(0.0, 0.0, 1.0)),
            checkmark_color: Color::from_rgb(0.0, 1.0, 0.0),
            border_radius: 9,
            border_width: 2,
            border_color: Color::BLACK,
        }
    }
}
