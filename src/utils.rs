pub type Filter = Vec<bool>;

pub fn apply_filter<I,E,F>(elements: E, filter: F) -> impl Iterator<Item = I>
    where E: Iterator<Item = I>,
          F: Iterator<Item = bool> {
    filter
        .zip(elements)
        .filter(|(include, _)| *include)
        .map(|(_, item)| item)
}