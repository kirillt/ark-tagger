use iced::{checkbox, Color, Background};

#[derive(Clone, Copy)]
pub struct CheckboxColor {
    pub color: Color,
}

impl checkbox::StyleSheet for CheckboxColor {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color { a: 0.5, ..self.color }),
            checkmark_color: Color::from_rgb(1.0, 0.0, 0.0),
            border_radius: 9,
            border_width: 2,
            border_color: Color::BLACK,
        }
    }

    fn hovered(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(self.color),
            checkmark_color: Color::from_rgb(0.0, 1.0, 0.0),
            border_radius: 9,
            border_width: 2,
            border_color: Color::BLACK,
        }
    }
}
