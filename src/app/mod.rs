mod tagger;
mod selector;
mod browser;
mod message;
mod order;
mod style;

use crate::model::{
    Model, id::Id,
    tag::HighlightedTag,
    location::Location
};
use crate::utils;

use message::{Message, TaggerMessage, BrowserMessage, FileMessage, DirMessage};

use tagger::Tagger;
use selector::Selector;
use browser::Browser;

use std::path::PathBuf;

use iced::{
    Application, Command,
    Container, Element,
    Column, Length
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

    type Flags = PathBuf;

    fn new(root: Self::Flags) -> (Self, Command<Message>) {
        let mut model = Model::new(root);
        let location = &mut model.location;
        let index = &mut model.index;

        let ids = location.files.iter()
            .map(|e| index.id(&e.path));

        let tags = model.database.sieved_tags(ids);
        let selector = Selector::new(tags);

        let browser = Browser::new(
            &location.directories,
            location.files.iter(),
            false);

        let tagger = Tagger::new();

        (RootWidget { model, tagger, selector, browser }, Command::none())
    }

    fn title(&self) -> String {
        "Hey there!".to_owned()
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        println!("Application::update(): {:?}", &msg);
        match msg {
            Message::TaggerMessage(TaggerMessage::TaggingActivated) => {
                let files_selection = self.browser.take_model_selection();
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

                        index.id(path)
                    });

                if database.insert(ids, &tag) {
                    //if we just tagged some file then the tag's bucket is not empty
                    self.selector.insert(HighlightedTag {
                        highlighted: true,
                        tag: &tag
                    });
                }

                //todo: update only sieve
                self.update_filter_and_sieve();
            },
            Message::TaggerMessage(msg) => {
                self.tagger.update(msg);
            },
            Message::SelectorMessage(msg) => {
                self.selector.update(msg);
                self.update_filter_and_sieve();
            },
            Message::BrowserMessage(BrowserMessage::AscendActivated) => {
                println!("\tAscending");
                let location = self.model.location.ascend(&mut self.model.index);
                self.change_location(location);
            },
            Message::BrowserMessage(BrowserMessage::DirMessage(i, DirMessage::DescendActivated)) => {
                println!("\tDescending into {}th entry", i);
                let location = self.model.location.descend(&mut self.model.index, i);
                self.change_location(location);
            }
            Message::BrowserMessage(BrowserMessage::FileMessage(i, FileMessage::ExecuteActivated)) => {
                println!("\tActivating {}th file", i);
                self.model.location.activate(i);
            },
            Message::BrowserMessage(BrowserMessage::OrderSelected(order)) => {
                self.browser.update(BrowserMessage::OrderSelected(order));
                self.update_filter_and_sieve(); //todo: this is a hack to cause refresh() with proper entries
            },
            Message::BrowserMessage(msg) => {
                self.browser.update(msg)
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let root: Element<Message> = Container::new::<Element<Message>>(
                Column::new()
                    .push(self.browser.view() //todo: scrolling doesn't look working
                        .map(|msg| { Message::BrowserMessage(msg) }))
                    .push(self.selector.view()
                        .map(|msg| { Message::SelectorMessage(msg) }))
                    .push(self.tagger.view()
                        .map(|msg| { Message::TaggerMessage(msg) }))
                    .align_items(iced::Align::Center)
                    .into())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into();

        #[cfg(debug_assertions)] {
            root.explain(iced::Color::BLACK)
        }
        #[cfg(not(debug_assertions))] {
            root
        }
    }
}

impl RootWidget {
    fn change_location(&mut self, location: Location) {
        let files = vec![];
        let files = files.iter();
        //todo: remove this hack

        self.browser = Browser::new(&location.directories, files,location.depth > 0);
        self.model.location = location;
        self.update_filter_and_sieve();
    }

    //todo: highlight directories with matching files
    //todo: directories with matching files should also affect highlighted tags
    fn update_filter_and_sieve(&mut self) {
        let location = &mut self.model.location;
        let index = &mut self.model.index;

        //todo: ids provision should be in Location
        //todo: ids and files in location must be synced
        let files = &location.files;
        let ids: Vec<Id> = files.iter()
            .map(|entry| index.id(entry.path.as_path()))
            .collect();

        let filter = self.model.database.filter(ids.iter().copied(), self.selector.selection());
        let filtered_ids = utils::apply_filter(ids.iter(), filter.iter().copied());
        let sieve = self.model.database.sieve(filtered_ids.copied());
        self.selector.highlight(sieve);

        let filtered_files = utils::apply_filter(files.iter(), filter.into_iter());
        self.browser.refresh(filtered_files);
    }
}