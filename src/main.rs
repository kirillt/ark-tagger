#[macro_use]
extern crate lazy_static;

use iced::{
    Application, Container, Element, Command,
    Row, Column, Font, Align, HorizontalAlignment, Length,
    Scrollable, Settings, Button, Checkbox, Text, TextInput, Color,
    button, scrollable, text_input,
};

use walkdir::WalkDir;
use crc32fast::Hasher;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File, DirEntry};
use std::io::Read;
use std::env;

lazy_static! {
    static ref ROOT: PathBuf =
        env::current_dir().unwrap()
            .canonicalize().unwrap();

    static ref DATA: PathBuf = {
        let mut path = ROOT.clone();
        path.push(DATA_NAME.to_owned());
        path
    };

    static ref DATA_NAME: &'static str = ".ark-tags";
}

fn main() {
    println!("Root: {:?}", *ROOT);
    Tagger::run(Settings::default())

    //let id: u32 = crc32(path);

    //let tags: Vec<String> = args.collect();
    //if !tags.is_empty() {
    //    for tag in tags.iter() {
    //        label(id, &path, &tag);
    //    }
    //}
}

type Id = u32;

enum Tagger {
    Loading, //todo: implement continuos loading/storing
    Loaded(State),
    Invalid(String)
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<Index,()>),
    LocationMessage(LocationMessage)
}

#[derive(Debug, Clone)]
struct Index {
    id_by_path: HashMap<PathBuf, Option<Id>>,
    path_by_id: HashMap<Id, PathBuf>
}

impl Index {
    async fn load() -> Result<Self, ()> {
        let mut id_by_path = HashMap::new();
        let mut path_by_id = HashMap::new();

        let walker = WalkDir::new(&*ROOT)
            .follow_links(false) //todo: enable when paths grouping by id is implemented
            .max_open(8) //small limit -- more memory spent
            //.sort_by()
            .into_iter()
            .filter_entry(|e| e.path() != *DATA);

        for entry in walker {
            let entry = entry.unwrap();
            let path  = entry.path();
            let path = path.strip_prefix(&*ROOT)
                .unwrap().into();

            id_by_path.insert(path, None);
        }
        println!("Total {} paths found", id_by_path.keys().len());

        Ok(Index { id_by_path, path_by_id })
    }
}

struct Location {
    path: PathBuf,
    entries: Vec<Entry>
}

impl Location {
    pub fn new(path: PathBuf) -> Self {
        let entries = fs::read_dir(&path).unwrap()
            .map(|e| e.unwrap())
            .map(|e| Entry {
                name: e.file_name().to_str().unwrap().to_owned(),
                path: e.path(),
                is_dir: e.file_type().unwrap().is_dir()
            })
            .filter(|e| e.name != *DATA_NAME
                        && e.path != *DATA)
            .collect();

        Location { path, entries }
    }
}

struct AscendButton {
    state: button::State,
    parent: PathBuf
}

struct LocationWidget {
    asc_button: Option<AscendButton>,
    entries: Vec<EntryWidget>,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
enum LocationMessage {
    AscendActivated(PathBuf),
    EntryMessage(usize, EntryMessage),
}

impl LocationWidget {
    fn new(location: Location) -> Self {
        let Location { path, entries } = location;

        let asc_button = if &path != &*ROOT {
            let mut path = path.to_path_buf();
            path.pop();
            Some(AscendButton {
                state: button::State::new(),
                parent: path
            })
        } else {
            None
        };

        LocationWidget {
            asc_button: asc_button,
            scroll: scrollable::State::new(),
            entries: entries.into_iter()
                         .map(|e| EntryWidget::new(e))
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
            _ => println!("LocationWidget received an unexpected message")
        }
    }

    fn view(&mut self) -> Element<LocationMessage> {
        match self {
            LocationWidget { asc_button, entries, scroll } => {
                let button: Element<LocationMessage> = match asc_button {
                    Some(AscendButton { state, parent }) => Button::new(state, Text::new("up"))
                        .on_press(LocationMessage::AscendActivated(parent.clone()))
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


struct Entry {
    name: String,
    path: PathBuf,
    is_dir: bool
}

struct EntryWidget {
    selected: bool,
    desc_button: button::State,
    entry: Entry,
}

#[derive(Debug, Clone)]
enum EntryMessage {
    DescendActivated(PathBuf),
    ExecuteActivated,
    Selected(bool)
}

impl EntryWidget {
    fn new(entry: Entry) -> Self {
        EntryWidget {
            selected: false,
            desc_button: button::State::new(),
            entry
        }
    }

    fn update(&mut self, msg: EntryMessage) {
        match msg {
            EntryMessage::Selected(value) => {
                self.selected = value;
            }
            EntryMessage::ExecuteActivated => {
                println!("\tExecuting {:?}", &self.entry.path);
            }
            _ => println!("EntryWidget received an unexpected message")
        }
    }

    fn view(&mut self) -> Element<EntryMessage> {
        let checkbox = Checkbox::new(self.selected, &self.entry.name,
                                     EntryMessage::Selected)
            .width(Length::Fill);

        let path = self.entry.path.clone();

        let button = if self.entry.is_dir {
            Button::new(&mut self.desc_button, Text::new("down"))
                .on_press(EntryMessage::DescendActivated(path))
        } else {
            Button::new(&mut self.desc_button, Text::new("open"))
                .on_press(EntryMessage::ExecuteActivated)
        };

        Row::new()
            .push(checkbox)
            .push(button)
            .into()
    }
}

struct State {
    location_widget: LocationWidget,
    index: Index,
}

impl State {
    fn new(index: Index) -> Self {
        let location = Location::new(ROOT.clone());

        State {
            location_widget: LocationWidget::new(location),
            index
        }
    }
}

impl Application for Tagger {
    type Executor = iced::executor::Default;
    type Message = Message;

    fn new() -> (Tagger, Command<Message>) {
        (Tagger::Loading, Command::perform(Index::load(), Message::Loaded))
    }

    fn title(&self) -> String {
        "Hey there!".to_owned()
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        match self {
            Tagger::Loading => {
                match msg {
                    Message::Loaded(Ok(index)) => {
                        *self = Tagger::Loaded(State::new(index));
                    },
                    Message::Loaded(Err(_)) => {
                        *self = Tagger::Invalid("Loading failed".to_owned());
                    },
                    _ => println!("Unexpected message received")
                }
            },
            Tagger::Loaded(state) => {
                println!("Application::update(): {:?}", &msg);
                match msg {
                    Message::LocationMessage(LocationMessage::AscendActivated(parent)) => {
                        println!("\tAscending");
                        state.location_widget = LocationWidget::new(Location::new(parent));
                    },
                    Message::LocationMessage(LocationMessage::EntryMessage(_, EntryMessage::DescendActivated(path))) => {
                        println!("\tDescending to {:?}", &path);
                        state.location_widget = LocationWidget::new(Location::new(path));

                    }
                    Message::LocationMessage(msg) => {
                        state.location_widget.update(msg)
                    },
                    _ => println!("Application received an unexpected message")
                }
            },
            Tagger::Invalid(_) => {
                println!("Unexpected message received");
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let root: Element<Message> = Container::new::<Element<Message>>(
            match self {
                Tagger::Loading => Text::new("Loading...").size(50).into(),
                Tagger::Invalid(err) => Text::new(err.clone()).size(50).into(),
                Tagger::Loaded(State { location_widget, index }) =>
                        location_widget.view()
                            .map(|msg| {
                                println!("Application::view(): a message from Location: {:?}", &msg);
                                Message::LocationMessage(msg)
                            })
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into();

        root.explain(Color::BLACK)
    }
}

fn path_view(path: &Path) -> Element<()> {
    Text::new(path.to_str().unwrap()).into()
}

// real stuff:

fn label(id: u32, target: &Path, tag: &str) {
    let mut path: PathBuf = DATA.clone();
    path.push(tag);

    fs::create_dir_all(&path).unwrap();
    path.push(format!("{}", id));

    File::create(&path).unwrap();
}

fn crc32(path: &Path) -> Id {
    //use sha1 in case of collisions

    let mut file = File::open(path).unwrap();

    let mut hasher = Hasher::new();
    //use reset() method when it will become more serious

    let mut buffer: Vec<u8> = vec![0; 512 * 1024];
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer);
    }

    hasher.finalize()
}
