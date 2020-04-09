mod tagger;
mod selector;
mod browser;

use crate::model::Model;
use crate::model::id::Id;
use crate::message::{Message, TaggerMessage, BrowserMessage, EntryMessage};

use tagger::Tagger;
use selector::Selector;
use browser::Browser;

use std::collections::HashSet;
use std::path::PathBuf;

use iced::{
    Application, Command,
    Container, Element,
    Column, Length,
    Color,
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
                let selection = &self.browser.selection;
                let entries = &self.model.location.entries;
                let tag = self.tagger.text.clone();
                println!("\tTagging {:?} with {:?}", selection, tag);

                let mut paths: Vec<Option<PathBuf>> = entries.iter()
                    .map(|e| Some(e.path.clone()))
                    .collect();

                let ids: HashSet<Id> = selection.clone().into_iter().map(|i| {
                    let path = paths[i].take().unwrap();
                    println!("\t\t{:?}", &path);

                    self.model.index.refresh(path)
                }).collect();

                if self.model.database.insert(ids, &tag) {
                    self.selector.push(tag);
                }
            },
            Message::TaggerMessage(msg) => {
                self.tagger.update(msg)
            },
            Message::SelectorMessage(msg) => {
                self.selector.update(msg)
            },
            Message::BrowserMessage(BrowserMessage::AscendActivated) => {
                println!("\tAscending");
                self.model.location = self.model.location.ascend(&mut self.model.index);
                self.browser = Browser::new(&self.model.location);
            },
            Message::BrowserMessage(BrowserMessage::EntryMessage(i, EntryMessage::DescendActivated)) => {
                println!("\tDescending into {}th entry", i);
                self.model.location = self.model.location.descend(&mut self.model.index, i);
                self.browser = Browser::new(&self.model.location);
            }
            Message::BrowserMessage(msg) => {
                self.browser.update(msg)
            },
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
