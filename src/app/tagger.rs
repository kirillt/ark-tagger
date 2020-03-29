use crate::message::TaggerMessage;

use iced::{text_input, button, Element, Text, TextInput, Button, Row};

pub struct Tagger {
    pub text: String,
    input: text_input::State,
    button: button::State
}

impl Tagger {
    pub fn new() -> Self {
        Tagger {
            text: "".to_owned(),
            input: text_input::State::focused(),
            button: button::State::new()
        }
    }

    pub fn update(&mut self, msg: TaggerMessage) {
        match msg {
            TaggerMessage::InputChanged(text) => self.text = text,
            _ => println!("Tagger received an unexpected message")
        }
    }

    pub fn view(&mut self) -> Element<TaggerMessage> {
        let input = TextInput::new(&mut self.input,
            "What tag do you want to put on your files?",
            &self.text, TaggerMessage::InputChanged)
                .on_submit(TaggerMessage::TaggingActivated);

        let button = Button::new(&mut self.button, Text::new("tag!"))
            .on_press(TaggerMessage::TaggingActivated);

        Row::new()
            .push(input)
            .push(button)
            .into()
    }
}
