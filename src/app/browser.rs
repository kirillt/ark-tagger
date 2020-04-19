use crate::model::entry::{DirEntry, FileEntry};
use super::message::{BrowserMessage, DirMessage, FileMessage};

use number_prefix::NumberPrefix;
use chrono::offset::Utc;
use chrono::DateTime;

use iced::{
    Element, Row, Column, Length,
    Scrollable, Button, Checkbox, Text, 
    button, scrollable,
};

use std::collections::BTreeSet;

pub struct Browser {
    selection: BTreeSet<usize>,
    file_widgets: Vec<FileWidget>,
    dir_widgets: Vec<DirWidget>,
    asc_button: Option<button::State>,
    scroll: scrollable::State,
}

impl Browser {
    pub fn new<'a, F>(directories: &Vec<DirEntry>, files: F, allow_ascend: bool) -> Self
    where F: Iterator<Item = &'a FileEntry> {
        let asc_button = if allow_ascend {
            Some(button::State::new())
        } else {
            None
        };

        let dir_widgets = directories.iter()
            .map(|e| DirWidget::new(e))
            .collect();

        let file_widgets = files
            .map(|e| FileWidget::new(e))
            .collect();

        Browser {
            selection: BTreeSet::new(),
            file_widgets,
            dir_widgets,
            asc_button,
            scroll: scrollable::State::new(),
        }
    }

    pub fn refresh<'a, F>(&mut self, files: F)
        where F: Iterator<Item = &'a FileEntry> {

        self.file_widgets = files
            .map(|e| FileWidget::new(e))
            .collect();

        self.scroll = scrollable::State::new();
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
            Browser { selection: _, file_widgets, dir_widgets, scroll, asc_button } => {
                debug_assert!(
                    file_widgets.iter()
                        .enumerate().filter_map(|(i, e)| {
                            if e.selected { Some(i) } else { None }
                        }).collect::<Vec<usize>>()
                    == self.selection.iter()
                        .cloned()
                        .collect::<Vec<usize>>());

                let list = Scrollable::new(scroll);
                let list = dir_widgets.iter_mut().enumerate().fold(list, |list, (i, dir)| {
                    list.push(dir.view().map(
                        move |msg| BrowserMessage::DirMessage(i, msg)))
                });
                let list = file_widgets.iter_mut().enumerate().fold(list, |list, (i, file)| {
                    list.push(file.view().map(
                        move |msg| BrowserMessage::FileMessage(i, msg)))
                });

                let mut column = Column::new();
                if let Some(state) = asc_button {
                    column = column.push(
                        Button::new(state, Text::new("up"))
                            .on_press(BrowserMessage::AscendActivated));
                }

                column.push(list).into()
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

impl DirWidget {
    fn new(entry: &DirEntry) -> Self {
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

struct FileWidget {
    name: String,
    meta: String,
    selected: bool,
    open_button: button::State,
}

impl FileWidget {
    fn new(entry: &FileEntry) -> Self {
        let date: DateTime<Utc> = entry.modified.clone().into();

        let size = match NumberPrefix::decimal(entry.size as f64) {
            NumberPrefix::Standalone(bytes) => {
                format!("{} bytes", bytes)
            }
            NumberPrefix::Prefixed(prefix, n) => {
                format!("{:.1} {}B", n, prefix)
            }
        };

        let meta = format!("size: {}\nmodified: {}",
           size, date.format("%d/%m/%Y %T"));

        FileWidget {
            name: entry.name.clone(),
            meta,
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

        let mut info = Column::new();
        for line in (&self.meta).split('\n') {
            info = info.push(Text::new(line.clone()).size(10));
        }

        let button =
            Button::new(&mut self.open_button, Text::new("open"))
                .on_press(FileMessage::ExecuteActivated);

        Row::new()
            .push(checkbox)
            .push(info)
            .push(button)
            .into()
    }
}