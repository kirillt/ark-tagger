use super::message::TaggerMessage;

use iced::{text_input, Element, TextInput};

pub struct Tagger {
    pub text: String,
    input: text_input::State
}

impl Tagger {
    pub fn new() -> Self {
        Tagger {
            text: "".to_owned(),
            input: text_input::State::focused()
        }
    }

    pub fn update(&mut self, msg: TaggerMessage) {
        match msg {
            TaggerMessage::InputChanged(text) => self.text = text,
            _ => println!("Tagger received an unexpected message")
        }
    }

    pub fn view(&mut self) -> Element<TaggerMessage> {
        TextInput::new(&mut self.input,
                "What tag do you want to put on your files?",
                &self.text, TaggerMessage::InputChanged)
            .on_submit(TaggerMessage::TaggingActivated)
            .into()
    }

    pub fn take_tag(&mut self) -> String {
        let result = self.text.clone();
        self.text = String::new();
        result
    }
}
