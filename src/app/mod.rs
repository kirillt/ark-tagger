mod selector;
mod browser;

use crate::model::Model;
use crate::message::{Message, SelectorMessage, LocationMessage, EntryMessage};
use crate::query;

use selector::Selector;
use browser::Browser;

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
    browser: Browser,
    selector: Selector
}

impl Application for RootWidget {
    type Executor = iced::executor::Default;
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        let model = Model::new();
        let browser = Browser::new(&model.location);
        let selector = Selector::new(&model.database);

        (RootWidget { model, browser, selector }, Command::none())
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
            Message::SelectorMessage(msg) => {
                self.selector.update(msg)
            },
            _ => println!("Application received an unexpected message")
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let root: Element<Message> = Container::new::<Element<Message>>(
                Column::new()
                    .push(self.selector.view()
                        .map(|msg| { Message::SelectorMessage(msg) }))
                    .push(self.browser.view()
                        .map(|msg| { Message::LocationMessage(msg) }))
                    .into())
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
