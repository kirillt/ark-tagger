use crate::model::{Location, Entry};

use crate::message::{LocationMessage, EntryMessage};

use iced::{
    Element, Row, Column, Length,
    Scrollable, Button, Checkbox, Text, 
    button, scrollable,
};

pub struct Browser {
    asc_button: Option<button::State>,
    entries: Vec<EntryWidget>,
    scroll: scrollable::State,
}

impl Browser {
    pub fn new(location: &Location) -> Self {
        let asc_button = if location.depth > 0 {
            Some(button::State::new())
        } else {
            None
        };

        Browser {
            asc_button: asc_button,
            scroll: scrollable::State::new(),
            entries: location.entries.iter()
                         .map(|e| EntryWidget::new(&e))
                         .collect(),
        }
    }

    pub fn update(&mut self, msg: LocationMessage) {
        println!("\tLocationMessage: {:?}", &msg);
        match msg {
            LocationMessage::EntryMessage(i, msg) => {
                if let Some(entry) = self.entries.get_mut(i) {
                    entry.update(msg);
                }
            },
            _ => println!("Browser received an unexpected message")
        }
    }

    pub fn view(&mut self) -> Element<LocationMessage> {
        match self {
            Browser { asc_button, entries, scroll } => {
                let button: Element<LocationMessage> = match asc_button {
                    Some(state) => Button::new(state, Text::new("up"))
                        .on_press(LocationMessage::AscendActivated)
                        .into(),
                    None => Text::new("You are in the root directory")
                        .into()
                };

                let list = entries
                    .iter_mut()
                    .enumerate()
                    .fold(Scrollable::new(scroll), |scrollable, (i, entry)|
                        scrollable.push(entry.view()
                            .map(move |msg| {
                                println!("Location::view(): a message from Entry: {:?}", &msg);
                                LocationMessage::EntryMessage(i, msg)
                            })));

                Column::new()
                    .push(button)
                    .push(list)
                    .into()
            }
        }
    }
}

struct EntryWidget {
    is_dir: bool,
    name: String,

    selected: bool,
    open_button: button::State,
}

impl EntryWidget {
    fn new(entry: &Entry) -> Self {
        EntryWidget {
            is_dir: entry.is_dir,
            name: entry.name.clone(),

            selected: false,
            open_button: button::State::new(),
        }
    }

    fn update(&mut self, msg: EntryMessage) {
        match msg {
            EntryMessage::Selected(value) => {
                self.selected = value;
            },
            _ => println!("EntryWidget received an unexpected message")
        }
    }

    fn view(&mut self) -> Element<EntryMessage> {
        let checkbox = Checkbox::new(
            self.selected, &self.name,
            EntryMessage::Selected)
                .width(Length::Fill);

        let button = if self.is_dir {
            Button::new(&mut self.open_button, Text::new("down"))
                .on_press(EntryMessage::DescendActivated)
        } else {
            Button::new(&mut self.open_button, Text::new("open"))
                .on_press(EntryMessage::ExecuteActivated)
        };

        Row::new()
            .push(checkbox)
            .push(button)
            .into()
    }
}
