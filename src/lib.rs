//! CVM distinct element estimation algorithm

pub fn estimate_distinct_elements(a_set: &[String], s: usize) -> u32 {
    const PRECISION: u32 = 100_000;
    fn flip_coin() -> u32 {
        rand::random::<u32>() % PRECISION
    }

    // D1
    let mut p = PRECISION;
    let mut b_set: Vec<(String, u32)> = Vec::new();

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
