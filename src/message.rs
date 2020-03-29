#[derive(Debug, Clone)]
pub enum Message {
    LocationMessage(LocationMessage)
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

