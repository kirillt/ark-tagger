mod tagger;
mod selector;
mod browser;

use crate::model::Model;
use crate::message::{Message, TaggerMessage, SelectorMessage, BrowserMessage, EntryMessage};
use crate::query;

use tagger::Tagger;
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
    tagger: Tagger,
    selector: Selector,
    browser: Browser,
}

impl Application for RootWidget {
    type Executor = iced::executor::Default;
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        let model = Model::new();
        let tagger = Tagger::new();
        let selector = Selector::new(&model.database);
        let browser = Browser::new(&model.location);

        (RootWidget { model, tagger, selector, browser }, Command::none())
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        println!("Application::update(): {:?}", &msg);
        match msg {
            Message::TaggerMessage(TaggerMessage::TaggingActivated) => {
                println!("\tTagging {:?} with {:?}", &self.browser.selection, &self.tagger.text);
            },
            Message::TaggerMessage(msg) => {
                self.tagger.update(msg)
            },
            Message::SelectorMessage(msg) => {
                self.selector.update(msg)
            },
            Message::BrowserMessage(BrowserMessage::AscendActivated) => {
                println!("\tAscending");
                self.model.location = self.model.location.ascend();
                self.browser = Browser::new(&self.model.location);
            },
            Message::BrowserMessage(BrowserMessage::EntryMessage(i, EntryMessage::DescendActivated)) => {
                println!("\tDescending into {}th entry", i);
                self.model.location = self.model.location.descend(i); 
                self.browser = Browser::new(&self.model.location);
            }
            Message::BrowserMessage(msg) => {
                self.browser.update(msg)
            },
            _ => println!("Application received an unexpected message")
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let root: Element<Message> = Container::new::<Element<Message>>(
                Column::new()
                    .push(self.tagger.view()
                        .map(|msg| { Message::TaggerMessage(msg) }))
                    .push(self.selector.view()
                        .map(|msg| { Message::SelectorMessage(msg) }))
                    .push(self.browser.view()
                        .map(|msg| { Message::BrowserMessage(msg) }))
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
