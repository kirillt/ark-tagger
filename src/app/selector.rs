use crate::model::{Tag, Database};
use crate::message::{SelectorMessage, TagMessage};
use crate::style::CheckboxColor;

use iced::{Element, Color, Checkbox, Row};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::collections::BTreeSet;

pub struct Selector {
    tags: Vec<TagWidget>,
    selection: BTreeSet<usize>
}

impl Selector {
    pub fn new(db: &Database) -> Self {
        let mut hasher = DefaultHasher::new();

        Selector {
            tags: db.ids_by_tag.keys()
                .map(|tag| TagWidget::new(tag, &mut hasher))
                .collect(),

            selection: BTreeSet::new()
        }
    }

    pub fn update(&mut self, msg: SelectorMessage) {
        println!("\tSelectorMessage: {:?}", &msg);
        match msg {
            SelectorMessage::TagMessage(i, msg) => {
                match &msg {
                    TagMessage::Selected(true) => { self.selection.insert(i); },
                    TagMessage::Selected(false) => { self.selection.remove(&i); },
                    _ => {}
                };
                println!("[ Tags selected: {:?} ]", &self.selection);

                if let Some(tag) = self.tags.get_mut(i) {
                    tag.update(msg);
                }
            },
            _ => println!("Selector received an unexpected message")
        }
    }

    pub fn view(&mut self) -> Element<SelectorMessage> {
        debug_assert!(
            self.tags.iter()
                .enumerate().filter_map(|(i, t)| {
                    if t.selected { Some(i) } else { None }
                }).collect::<Vec<usize>>()
            == self.selection.iter()
                .cloned()
                .collect::<Vec<usize>>());

        self.tags
            .iter_mut()
            .enumerate()
            .fold(Row::new(), |row, (i, tag)|
                row.push(tag.view()
                    .map(move |msg| {
                        println!("Selector::view(): a message from Tag: {:?}", &msg);
                        SelectorMessage::TagMessage(i, msg)
                    })))
            .into()
    }
}

pub struct TagWidget {
    name: String,
    selected: bool,
    color: CheckboxColor,
}

impl TagWidget {
    fn new<H: Hasher>(tag: &Tag, hasher: &mut H) -> Self {
        (*tag).hash(hasher);
        let hash = hasher.finish(); //using the same hasher intentionally

        let f32_from_u64 = |x: u64| {
            255f32 / (x as f32)
        };

        let color = CheckboxColor {
            color: Color::from_rgb(
                f32_from_u64(hash % 256),
                f32_from_u64((hash << 2) % 256),
                f32_from_u64((hash << 4) % 256))
        };

        TagWidget {
            name: tag.to_owned(),
            selected: false,
            color,
        }
    }

    fn update(&mut self, msg: TagMessage) {
        match msg {
            TagMessage::Selected(value) => {
                self.selected = value;
            },
            _ => println!("TagWidget received an unexpected message")
        }
    }

    fn view(&mut self) -> Element<TagMessage> {
        Checkbox::new(
            self.selected, &self.name,
            TagMessage::Selected)
                .style(self.color)
            .into()
    }
}
