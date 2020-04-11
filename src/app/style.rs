use iced::{checkbox, Color, Background};

#[derive(Clone, Copy)]
pub struct TagStyle {
    pub color: Option<Color>,
}

impl checkbox::StyleSheet for TagStyle {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        let color = self.color.unwrap_or(Color::WHITE);

        checkbox::Style {
            background: Background::Color(Color { a: 0.5, ..color }),
            checkmark_color: Color::from_rgb(1.0, 0.0, 0.0),
            border_radius: 9,
            border_width: 2,
            border_color: Color::BLACK,
        }
    }

    fn hovered(&self, _is_checked: bool) -> checkbox::Style {
        let color = self.color.unwrap_or(Color::WHITE);

        checkbox::Style {
            background: Background::Color(color),
            checkmark_color: Color::from_rgb(0.0, 1.0, 0.0),
            border_radius: 9,
            border_width: 2,
            border_color: Color::BLACK,
        }
    }
}
