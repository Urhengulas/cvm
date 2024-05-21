use std::fs;

fn main() {
    let words = read_words();

    let mut unique_words = CountList::new();
    let mut round_counter = 0;
    const N: usize = 1000;

    for word in words {
        match unique_words.insert(word) {
            // the word is known already
            Some(idx) => {
                for _ in 0..round_counter {
                    // flip the coin to see if we should keep the word
                    if flip_coin() {
                        unique_words.remove(idx);
                        break;
                    }
                }
            }
            None => {} // it's a new word
        }

        // drop random words when list is full
        if unique_words.len() >= N {
            unique_words.end_round();
            round_counter += 1;
        }
    }

    dbg!(unique_words.len(), round_counter);
}

fn read_words() -> Vec<String> {
    let text = fs::read_to_string("hamlet.txt").unwrap();
    let text = text.replace(['.', ',', '?', ':', ';', '!'], " ");
    let text = text.replace("--", " ");
    let text = text.to_lowercase();
    text.split_whitespace().map(Into::into).collect()
}

#[derive(Debug, Default)]
struct CountList(Vec<String>);

impl CountList {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, value: String) -> Option<usize> {
        if let Some(idx) = self.0.iter().position(|x| x == &value) {
            Some(idx)
        } else {
            self.0.push(value);
            None
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn end_round(&mut self) {
        self.0.retain(|_| flip_coin());
    }

    fn remove(&mut self, index: usize) {
        self.0.swap_remove(index);
    }
}

fn flip_coin() -> bool {
    rand::random()
}
