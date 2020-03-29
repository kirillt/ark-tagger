use crate::model::{Model, Index, Location, Entry};
use crate::message::{Message, LocationMessage, EntryMessage};
use crate::query;

use std::path::PathBuf;

use iced::{
    Application,
    Container, Element, Command,
    Row, Column, Length, Scrollable,
    Button, Checkbox, Text, TextInput, Color,
    button, scrollable, text_input,
};

pub struct RootWidget {
    model: Model,
    browser: Browser
}

impl Application for RootWidget {
    type Executor = iced::executor::Default;
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        let model = query::init_model();
        let browser = Browser::new(&model.location);

        (RootWidget { model, browser }, Command::none())
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        println!("Application::update(): {:?}", &msg);
        match msg {
            Message::LocationMessage(LocationMessage::AscendActivated) => {
                println!("\tAscending");
                self.model.location = self.model.location.ascend();
                self.browser = Browser::new(&self.model.location);
            },
            Message::LocationMessage(LocationMessage::EntryMessage(i, EntryMessage::DescendActivated)) => {
                println!("\tDescending into {}th entry", i);
                self.model.location = self.model.location.descend(i); 
                self.browser = Browser::new(&self.model.location);
            }
            Message::LocationMessage(msg) => {
                self.browser.update(msg)
            },
            _ => println!("Application received an unexpected message")
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let root: Element<Message> = Container::new::<Element<Message>>(
                self.browser
                    .view()
                    .map(|msg| { Message::LocationMessage(msg) }))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into();

        root.explain(Color::BLACK)
    }

    fn title(&self) -> String {
        "Hey there!".to_owned()
    }
}


struct AscendButton {
    state: button::State,
}

struct Browser {
    asc_button: Option<AscendButton>,
    entries: Vec<EntryWidget>,
    scroll: scrollable::State,
}

impl Browser {
    fn new(location: &Location) -> Self {
        let asc_button = if location.depth > 0 {
            Some(AscendButton {
                state: button::State::new()
            })
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

    fn update(&mut self, msg: LocationMessage) {
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

    fn view(&mut self) -> Element<LocationMessage> {
        match self {
            Browser { asc_button, entries, scroll } => {
                let button: Element<LocationMessage> = match asc_button {
                    Some(AscendButton { state }) => Button::new(state, Text::new("up"))
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
            }
            _ => println!("EntryWidget received an unexpected message")
        }
    }

    fn view(&mut self) -> Element<EntryMessage> {
        let checkbox = Checkbox::new(self.selected, &self.name,
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
