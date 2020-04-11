pub type Tag = String;

pub struct HighlightedTag<'a> {
    pub highlighted: bool,
    pub tag: &'a Tag
}