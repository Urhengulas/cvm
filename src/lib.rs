//! CVM distinct element estimation algorithm
//!
//! Implementation after "The CVM Algorithm for Estimating Distinct Elements in
//! Streams" by Don Knuth, Stanford Computer Science Department (25 May 2023,
//! revised 29 December 2023).

use std::{collections::HashMap, hash::Hash};

pub fn estimate_distinct_elements<T>(a_set: &[T], s: usize) -> u32
where
    T: Eq + Hash,
{
    // D1
    let mut buf = CvmBuffer::<_, 1_000>::new(s);

    // D2 & D3
    for a_t in a_set {
        buf.insert(a_t);
    }
    buf.estimate()
}

#[derive(Clone, Debug)]
pub struct CvmBuffer<T, const PRECISION: u32 = 1_000> {
    buf: HashMap<T, u32>,
    s: usize,
    p: u32,
}

impl<T, const PRECISION: u32> CvmBuffer<T, PRECISION> {
    pub fn new(s: usize) -> Self {
        Self {
            buf: HashMap::with_capacity(s),
            s,
            p: PRECISION,
        }
    }

    pub fn estimate(&self) -> u32 {
        (self.buf.len() as u32) * PRECISION / self.p
    }

    fn flip_coin() -> u32 {
        rand::random::<u32>() % PRECISION
    }
}

impl<T, const PRECISION: u32> CvmBuffer<T, PRECISION>
where
    T: Clone + Eq + Hash,
{
    pub fn insert(&mut self, a_t: T) {
        let u_t = Self::flip_coin();
        if u_t >= self.p {
            self.buf.remove(&a_t);
        } else if self.buf.len() < self.s {
            self.buf.insert(a_t, u_t);
        } else {
            self.update_p(a_t, u_t)
        }
    }

    /// Get the entry with the highest value
    fn max(&mut self) -> (T, u32) {
        let a_max = self.buf.iter().max_by_key(|a| a.1).unwrap().0;
        self.buf.remove_entry(&a_max.clone()).unwrap()
    }

    fn update_p(&mut self, a_t: T, u_t: u32) {
        let (a_max, u_max) = self.max();

        if u_t > u_max {
            self.p = u_t;
            self.buf.insert(a_max, u_max);
        } else {
            self.p = u_max;
            self.buf.insert(a_t, u_t);
        }
    }
}
