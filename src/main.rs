//! CVM distinct element estimation algorithm
//!
//! - `A` - stream of elements {a1, a2, ..., am} or set of elements in the stream
//! - `|A|` - the size of that set, with multiplicities ignored
//! - `At` - the first t elements of that stream, ignoring multiplicity
//! - `Bt` - buffer B that remembers a randomly chosen subset of thise elements at time t
//! - `pt` - |Bt|/pt is an unbiased estimate of |At|
//! - `s` - maximum buffer size

use std::{collections::HashSet, fs};

fn main() {
    let input = read_words();
    let estimate = estimate_distinct_elements(&input);
    let correct = input.into_iter().collect::<HashSet<_>>().len();
    dbg!(estimate, correct);
}

fn estimate_distinct_elements(a_set: &[String]) -> u32 {
    const PRECISION: u32 = 100_000;
    fn flip_coin() -> u32 {
        rand::random::<u32>() % PRECISION
    }

    // D1
    let mut p = PRECISION;
    let mut b_set: Vec<(String, u32)> = Vec::new();
    let s = 500;

    // D2 & D3
    for a_t in a_set {
        // D4
        b_set.retain(|(a, _)| a != a_t);

        // D5
        let u_t = flip_coin();
        if u_t >= p {
            continue;
        }
        if b_set.len() < s {
            b_set.push((a_t.clone(), u_t));
            continue;
        }

        // D6
        let (a_max, u_max) = b_set.iter_mut().max_by_key(|b| b.1).unwrap();
        if u_t > *u_max {
            p = u_t;
        } else {
            p = *u_max;
            *a_max = a_t.clone();
            *u_max = u_t;
        }
    }
    (b_set.len() as u32) * PRECISION / p
}

fn read_words() -> Vec<String> {
    let text = fs::read_to_string("hamlet.txt").unwrap();
    let text = text.replace(['.', ',', '?', ':', ';', '!'], " ");
    let text = text.replace("--", " ");
    let text = text.to_lowercase();
    text.split_whitespace().map(Into::into).collect()
}
