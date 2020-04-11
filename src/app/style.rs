use iced::{checkbox, Color, Background};

#[derive(Clone, Copy)]
pub struct TagStyle {
    pub color: Option<Color>,
}

impl TagStyle {
    fn colors(&self) -> (Color, Color) {
        let background_color = self.color.clone().unwrap_or(Color::WHITE);
        let checkmark_color = self.color
            .map(|color| boost_all(color, 0.3))
            .unwrap_or(Color::BLACK);

        (background_color, checkmark_color)
    }
}

impl checkbox::StyleSheet for TagStyle {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        let (background_color, checkmark_color) = self.colors();
        let background_color = boost_blue(background_color, 2.0);
        let checkmark_color = boost_green(checkmark_color, 2.0);

        checkbox::Style {
            background: Background::Color(with_alpha(background_color, 0.7)),
            checkmark_color: with_alpha(checkmark_color, 0.7),
            border_radius: 9,
            border_width: 2,
            border_color: Color::BLACK,
        }
    }

    fn hovered(&self, _is_checked: bool) -> checkbox::Style {
        let (background_color, checkmark_color) = self.colors();
        let background_color = boost_green(background_color, 2.0);
        let checkmark_color = boost_red(checkmark_color, 2.0);

        checkbox::Style {
            background: Background::Color(background_color),
            checkmark_color,
            border_radius: 9,
            border_width: 2,
            border_color: Color::BLACK,
        }
    }
}

fn boost_all(color: Color, factor: f32) -> Color {
    boost_red(boost_green(boost_blue(color,
         factor), factor), factor)
}

fn boost_red(color: Color, factor: f32) -> Color {
    Color { r: color.r * factor, ..color }
}

fn boost_green(color: Color, factor: f32) -> Color {
    Color { g: color.g * factor, ..color }
}

fn boost_blue(color: Color, factor: f32) -> Color {
    Color { b: color.b * factor, ..color }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}