//! CVM distinct element estimation algorithm
//!
//! Implementation after "The CVM Algorithm for Estimating Distinct Elements in
//! Streams" by Don Knuth, Stanford Computer Science Department (25 May 2023,
//! revised 29 December 2023).

const PRECISION: u32 = 1_000;

pub fn estimate_distinct_elements<T>(a_set: &[T], s: usize) -> u32
where
    T: Clone + PartialEq,
{
    // D1
    let mut p = PRECISION;
    let mut b_set = Vec::new();

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

fn flip_coin() -> u32 {
    rand::random::<u32>() % PRECISION
}
