#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    AsIs,
    BySize,
    ByCreatedDate,
    ByModifiedDate,
    ByAccessedDate,
}

use Order::*;

impl Order {
    pub fn all() -> [(Order, &'static str); 5] {
        [(AsIs, "no order"),
            (BySize, "by size"),
            (ByCreatedDate, "by creation"),
            (ByModifiedDate, "by modification"),
            (ByAccessedDate, "by last access")]
    }
}