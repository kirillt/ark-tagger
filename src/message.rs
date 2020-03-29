#[derive(Debug, Clone)]
pub enum Message {
    LocationMessage(LocationMessage),
    SelectorMessage(SelectorMessage)
}

#[derive(Debug, Clone)]
pub enum LocationMessage {
    AscendActivated,
    EntryMessage(usize, EntryMessage),
}

#[derive(Debug, Clone)]
pub enum EntryMessage {
    DescendActivated,
    ExecuteActivated,
    Selected(bool)
}

#[derive(Debug, Clone)]
pub enum SelectorMessage {
    TagMessage(usize, TagMessage)
}

#[derive(Debug, Clone)]
pub enum TagMessage {
    Selected(bool)
}
