use crate::model::tag::{Tag, HighlightedTag};
use crate::message::{SelectorMessage, TagMessage};

use super::style::TagStyle;

use iced::{Element, Color, Checkbox, Row};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Selector {
    tag_widgets: Vec<TagWidget>,
    selection: Vec<bool>,
    hasher: Box<dyn Hasher>
}

impl Selector {
    pub fn new<'a, T>(tags: T) -> Self
    where T: Iterator<Item = HighlightedTag<'a>> {
        let mut hasher = DefaultHasher::new();

        let widgets: Vec<TagWidget> = tags
            .map(|tag| TagWidget::new(tag, &mut hasher))
            .collect();
        let n = widgets.len();

        Selector {
            tag_widgets: widgets,
            selection: vec![false; n],
            hasher: Box::new(hasher)
        }
    }

    pub fn insert(&mut self, tag: HighlightedTag) {
        let n = self.tag_widgets.len();
        let pos = self.tag_widgets.iter()
            .enumerate()
            .find(|(_, widget)| {
                debug_assert!(widget.tag != *(tag.tag));
                widget.tag > *(tag.tag)
            })
            .map(|(i, _)| i)
            .unwrap_or(n);

        self.tag_widgets.insert(pos, TagWidget::new(tag, &mut self.hasher));
        self.selection.insert(pos, false);
    }

    pub fn update(&mut self, msg: SelectorMessage) {
        println!("\tSelectorMessage: {:?}", &msg);
        match msg {
            SelectorMessage::TagMessage(i, msg) => {
                match &msg {
                    TagMessage::Selected(value) => { self.selection[i] = *value; },
                };
                println!("[ Tags selected: {:?} ]", &self.selection);

                if let Some(tag) = self.tag_widgets.get_mut(i) {
                    tag.update(msg);
                }
            },
        }
    }

    pub fn view(&mut self) -> Element<SelectorMessage> {
        debug_assert!(
            self.tag_widgets.iter()
                .enumerate()
                .filter(|(_, widget)| widget.selected)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>()
            == self.selection.iter()
                .enumerate()
                .filter(|(_, selected)| **selected)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>());

        self.tag_widgets
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

    pub fn highlight<S>(&mut self, sieve: S)
    where S: Iterator<Item = bool> {
        self.tag_widgets.iter_mut()
            .zip(sieve.into_iter())
            .for_each(|(mut widget, highlighted)| {
                widget.highlighted = highlighted;
            })
    }

    pub fn selection(&self) -> impl Iterator<Item = &Tag> {
        self.selection.iter()
            .zip(self.tag_widgets.iter())
            .filter(|(i, _)| **i)
            .map(|(_, widget)| &widget.tag)
    }
}

pub struct TagWidget {
    tag: Tag,
    selected: bool,
    highlighted: bool,
    color: Color,
}

impl TagWidget {
    fn new<H: Hasher>(tag: HighlightedTag, hasher: &mut H) -> Self {
        let HighlightedTag { highlighted, tag } = tag;
        let tag = tag.to_owned();

        tag.hash(hasher);
        let hash = hasher.finish(); //using the same hasher intentionally

        let f32_from_u64 = |x: u64| {
            x as f32 / 255f32
        };

        let color = Color::from_rgb(
            f32_from_u64(hash % 256),
            f32_from_u64((hash << 2) % 256),
            f32_from_u64((hash << 4) % 256));

        let selected = false;
        TagWidget { tag, selected, highlighted, color }
    }

    fn update(&mut self, msg: TagMessage) {
        match msg {
            TagMessage::Selected(value) => {
                self.selected = value;
            },
        }
    }

    fn view(&mut self) -> Element<TagMessage> {
        let color = if self.highlighted { Some(self.color) } else { None };

        Checkbox::new(
            self.selected, &self.tag,
            TagMessage::Selected)
                .style(TagStyle { color })
            .into()
    }
}