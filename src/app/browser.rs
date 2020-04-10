use crate::model::database::Filter;
use crate::model::{Location, Entry};
use crate::model::tag::{Tag, Tags};
use crate::message::{BrowserMessage, DirMessage, FileMessage};

use iced::{
    Element, Row, Column, Length,
    Scrollable, Button, Checkbox, Text, 
    button, scrollable,
};

use std::collections::{BTreeSet, BTreeMap};

pub struct Browser {
    filter: Option<Vec<usize>>,
    selection: BTreeSet<usize>,
    file_widgets: Vec<FileWidget>,
    dir_widgets: Vec<DirWidget>,
    asc_button: Option<button::State>,
    scroll: scrollable::State,
}

impl Browser {
    pub fn new(location: &Location, filter: Option<Filter>) -> Self {
        let asc_button = if location.depth > 0 {
            Some(button::State::new())
        } else {
            None
        };

        let dir_widgets = location.directories.iter()
            .map(|e| DirWidget::new(e))
            .collect();

        let (file_widgets, filter) = if let Some(filter) = filter {
            let file_widgets = location.files.iter()
                .enumerate()
                .filter(|(i, _)| filter.contains(i))
                .map(|(_, e)| FileWidget::new(e))
                .collect();

            let filter = filter.into_iter().collect();
            //this vector will be used for restoring original indices

            (file_widgets, Some(filter))
        } else {
            let file_widgets = location.files.iter()
                .map(|e| FileWidget::new(e))
                .collect();

            (file_widgets, None)
        };

        Browser {
            filter,
            selection: BTreeSet::new(),
            file_widgets,
            dir_widgets,
            asc_button,
            scroll: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, msg: BrowserMessage) {
        println!("\tBrowserMessage: {:?}", &msg);
        match msg {
            BrowserMessage::FileMessage(i, msg) => {
                match &msg {
                    FileMessage::Selected(true) => { self.selection.insert(i); }
                    FileMessage::Selected(false) => { self.selection.remove(&i); }
                    _ => {}
                };
                println!("[ Paths selected: {:?} ]", &self.selection);

                if let Some(file_widget) = self.file_widgets.get_mut(i) {
                    file_widget.update(msg);
                }
            },
            _ => println!("Browser received an unexpected message")
        }
    }

    pub fn view(&mut self) -> Element<BrowserMessage> {
        match self {
            Browser { filter: _, selection: _, file_widgets, dir_widgets, scroll, asc_button } => {
                debug_assert!(
                    file_widgets.iter()
                        .enumerate().filter_map(|(i, e)| {
                            if e.selected { Some(i) } else { None }
                        }).collect::<Vec<usize>>()
                    == self.selection.iter()
                        .cloned()
                        .collect::<Vec<usize>>());

                let button: Element<BrowserMessage> = match asc_button {
                    Some(state) => Button::new(state, Text::new("up"))
                        .on_press(BrowserMessage::AscendActivated)
                        .into(),
                    None => Text::new("You are in the root directory")
                        .into()
                };

                let list = Scrollable::new(scroll);
                let list = dir_widgets.iter_mut().enumerate().fold(list, |list, (i, dir)| {
                    list.push(dir.view().map(
                        move |msg| BrowserMessage::DirMessage(i, msg)))
                });
                let list = file_widgets.iter_mut().enumerate().fold(list, |list, (i, file)| {
                    list.push(file.view().map(
                        move |msg| BrowserMessage::FileMessage(i, msg)))
                });

                Column::new()
                    .push(button)
                    .push(list)
                    .into()
            }
        }
    }

    pub fn take_selection(&mut self) -> BTreeSet<usize> {
        let result = self.selection.clone();
        self.selection = BTreeSet::new();

        for file in self.file_widgets.iter_mut() {
            file.selected = false;
        }

        result
    }
}

struct DirWidget {
    name: String,
    descend_button: button::State,
}

struct FileWidget {
    name: String,
    selected: bool,
    open_button: button::State,
}

impl DirWidget {
    fn new(entry: &Entry) -> Self {
        DirWidget {
            name: entry.name.clone(),
            descend_button: button::State::new(),
        }
    }

    fn view(&mut self) -> Element<DirMessage> {
        let label = Text::new(&self.name)
            .width(Length::Fill);

        let button =
            Button::new(&mut self.descend_button, Text::new("down"))
                .on_press(DirMessage::DescendActivated);

        Row::new()
            .push(label)
            .push(button)
            .into()
    }
}

impl FileWidget {
    fn new(entry: &Entry) -> Self {
        FileWidget {
            name: entry.name.clone(),
            selected: false,
            open_button: button::State::new(),
        }
    }

    fn update(&mut self, msg: FileMessage) {
        match msg {
            FileMessage::Selected(value) => {
                self.selected = value;
            },
            _ => println!("FileWidget received an unexpected message")
        }
    }

    fn view(&mut self) -> Element<FileMessage> {
        let checkbox = Checkbox::new(
            self.selected, &self.name,
            FileMessage::Selected)
            .width(Length::Fill);

        let button =
            Button::new(&mut self.open_button, Text::new("open"))
                .on_press(FileMessage::ExecuteActivated);

        Row::new()
            .push(checkbox)
            .push(button)
            .into()
    }
}