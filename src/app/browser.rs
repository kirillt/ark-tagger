use crate::model::entry::{DirEntry, FileEntry};
use super::message::{BrowserMessage, DirMessage, FileMessage};
use super::order::Order;

use number_prefix::NumberPrefix;
use chrono::offset::Utc;
use chrono::DateTime;

use iced::{
    Element, Row, Column, Length,
    Scrollable, Button, Checkbox, Radio, Text,
    button, scrollable,
};

use std::collections::BTreeSet;

pub struct Browser {
    order: Order,
    ordering: Option<Vec<usize>>,
    selection: BTreeSet<usize>,
    file_widgets: Vec<FileWidget>,
    dir_widgets: Vec<DirWidget>,
    asc_button: Option<button::State>,
    dir_scroll: scrollable::State,
    file_scroll: scrollable::State,
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
            order: Order::AsIs,
            ordering: None,
            selection: BTreeSet::new(),
            file_widgets,
            dir_widgets,
            asc_button,
            dir_scroll: scrollable::State::new(),
            file_scroll: scrollable::State::new(),
        }
    }

    pub fn refresh<'a, F>(&mut self, files: F)
        where F: Iterator<Item = &'a FileEntry> {

        if self.order != Order::AsIs {
            println!("\tBuffering and ordering entries");

            let mut files: Vec<(usize, &FileEntry)> = files.enumerate().collect();
            files.sort_by_key(|(_, file)| match self.order {
                Order::BySize => file.size,
                Order::ByCreatedDate => file.created_secs(),
                Order::ByModifiedDate => file.modified_secs(),
                Order::ByAccessedDate => file.accessed_secs(),
                _ => panic!("redundant buffering")
            });
            let (ordering, files): (Vec<_>, Vec<_>) = files.into_iter().unzip();
            self.ordering = Some(ordering);

            self.file_widgets = files
                .into_iter()
                .map(|e| FileWidget::new(e))
                .collect();
        } else {
            self.file_widgets = files
                .map(|e| FileWidget::new(e))
                .collect();
        };

        self.file_scroll = scrollable::State::new();
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
            BrowserMessage::OrderSelected(order) => {
                self.order = order;
            },
            _ => println!("Browser received an unexpected message")
        }
    }

    pub fn view(&mut self) -> Element<BrowserMessage> {
        match self {
            Browser {
                order,
                ordering: _,
                selection: _,
                file_widgets,
                dir_widgets,
                asc_button,
                dir_scroll,
                file_scroll,
            } => {
                debug_assert!(
                    file_widgets.iter()
                        .enumerate().filter_map(|(i, e)| {
                            if e.selected { Some(i) } else { None }
                        }).collect::<Vec<usize>>()
                    == self.selection.iter()
                        .cloned()
                        .collect::<Vec<usize>>());

                let directories = Scrollable::new(dir_scroll);
                let directories = dir_widgets.iter_mut().enumerate().fold(directories, |list, (i, dir)| {
                    list.push(dir.view().map(
                        move |msg| BrowserMessage::DirMessage(i, msg)))
                });

                let files = Scrollable::new(file_scroll);
                let files = file_widgets.iter_mut().enumerate().fold(files, |list, (i, file)| {
                    list.push(file.view().map(
                        move |msg| BrowserMessage::FileMessage(i, msg)))
                });

                let order_selector = Order::all().iter().cloned().fold(
                Row::new(), |choices, (option, label)| {
                    choices.push(Radio::new(
                        option,
                        label.to_owned(),
                        Some(*order),
                        BrowserMessage::OrderSelected))
                });

                let mut column = Column::new();

                if let Some(state) = asc_button {
                    column = column.push(
                        Button::new(state, Text::new("up"))
                            .on_press(BrowserMessage::AscendActivated));
                }
                column
                    .push(directories)
                    .push(order_selector)
                    .push(files)
                    .height(Length::Fill)
                    .into()
            }
        }
    }

    //the return selection is converted according selected Order
    pub fn take_model_selection(&mut self) -> BTreeSet<usize> {
        let result = self.selection.clone();
        self.selection = BTreeSet::new();
        //todo: optimize

        for file in self.file_widgets.iter_mut() {
            file.selected = false;
        }

        if let Some(ordering) = &self.ordering {
            let result: BTreeSet<usize> = result.into_iter()
                .map(|i| ordering[i])
                .collect();

            self.ordering = None; //todo: is it really should be used only once?
            result
        } else {
            result
        }
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