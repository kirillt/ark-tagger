#[derive(Debug, Clone)]
pub enum Message {
    TaggerMessage(TaggerMessage),
    SelectorMessage(SelectorMessage),
    BrowserMessage(BrowserMessage),
}

#[derive(Debug, Clone)]
pub enum TaggerMessage {
    TaggingActivated,
    InputChanged(String)
}

#[derive(Debug, Clone)]
pub enum BrowserMessage {
    AscendActivated,
    DirMessage(usize, DirMessage),
    FileMessage(usize, FileMessage),
}

#[derive(Debug, Clone)]
pub enum DirMessage {
    DescendActivated,
}

#[derive(Debug, Clone)]
pub enum FileMessage {
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
