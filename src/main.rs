use std::{collections::HashSet, fs};

fn main() {
    let words = read_words();
    let unique_words = words.into_iter().collect::<HashSet<_>>();
    dbg!(unique_words.len());
}

fn read_words() -> Vec<String> {
    let text = fs::read_to_string("hamlet.txt").unwrap();
    let text = text.replace(['.', ',', '?', ':', ';', '!'], " ");
    let text = text.replace("--", " ");
    let text = text.to_lowercase();
    text.split_whitespace().map(Into::into).collect()
}
