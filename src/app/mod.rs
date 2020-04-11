mod tagger;
mod selector;
mod browser;
mod style;

use crate::model::{Model, id::Id, tag::HighlightedTag};
use crate::message::{Message, TaggerMessage, BrowserMessage, DirMessage};

use tagger::Tagger;
use selector::Selector;
use browser::Browser;

use std::path::PathBuf;

use iced::{
    Application, Command,
    Container, Element,
    Column, Length,
    Color,
};

pub struct RootWidget<'a> {
    model: Model<'a>,
    tagger: Tagger,
    selector: Selector,
    browser: Browser,
}

impl<'a> Application for RootWidget<'a> {
    type Executor = iced::executor::Default;
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        let mut model = Model::new();
        let location = &mut model.location;
        let index = &mut model.index;

        let ids = location.files.iter()
            .map(|e| index.id_by_path[&e.path].unwrap());

        let tags = model.database.sieved_tags(ids);

        let selector = Selector::new(tags);
        let browser = Browser::new(&location, None);

        let tagger = Tagger::new();

        (RootWidget { model, tagger, selector, browser }, Command::none())
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        println!("Application::update(): {:?}", &msg);
        match msg {
            Message::TaggerMessage(TaggerMessage::TaggingActivated) => {
                let files_selection = self.browser.take_selection();
                let tag = self.tagger.take_tag();

                println!("\tTagging {:?} with {:?}", files_selection, tag);

                let file_entries = &self.model.location.files;
                let file_paths: Vec<PathBuf> = file_entries.iter()
                    .map(|e| e.path.clone())
                    .collect();

                let index = &mut self.model.index;
                let database = &mut self.model.database;

                let ids = files_selection.into_iter()
                    .map(|i| {
                        let path = &file_paths[i];
                        println!("\t\t{:?}", &path);

                        index.provide(path)
                    });

                if database.insert(ids, &tag) {
                    //if we just tagged some file then the tag's bucket is not empty
                    self.selector.insert(HighlightedTag {
                        highlighted: true,
                        tag: &tag
                    });
                }
            },
            Message::TaggerMessage(msg) => {
                self.tagger.update(msg)
            },
            Message::SelectorMessage(msg) => {
                self.selector.update(msg);

                let location = &mut self.model.location;
                let index = &mut self.model.index;

                let ids: Vec<Id> = location.files.iter()
                    .map(|entry| index.provide(entry.path.as_path()))
                    .collect();

                let filter = self.model.database.filter(ids.iter().copied(), self.selector.selection());

                self.browser = Browser::new(&self.model.location, Some(filter));

                let sieve = self.model.database.sieve(ids.iter().copied());

                self.selector.highlight(sieve);
            },
            Message::BrowserMessage(BrowserMessage::AscendActivated) => {
                println!("\tAscending");
                self.model.location = self.model.location.ascend(&mut self.model.index);
                self.browser = Browser::new(&self.model.location, None);
            },
            Message::BrowserMessage(BrowserMessage::DirMessage(i, DirMessage::DescendActivated)) => {
                println!("\tDescending into {}th entry", i);
                self.model.location = self.model.location.descend(&mut self.model.index, i);
                self.browser = Browser::new(&self.model.location, None);
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
