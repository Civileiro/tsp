use std::collections::HashMap;

use crate::{
    grid::Grid,
    tsp::{Tsp, TspSolution},
};

pub struct HeldKarp {}

impl TspSolution for HeldKarp {
    fn solve(tsp: &Tsp) -> usize {
        let size = tsp.cities.len();
        let mut distances = Grid::new(size, size, &0usize);
        for i in 0..size {
            for j in 0..size {
                if i == j {
                    continue;
                }
                *distances.get_mut(i, j) = tsp.cities[i].distance(&tsp.cities[j]);
            }
        }
        let mut g = HashMap::<(usize, usize), (usize, usize)>::new();
        let n = distances.height;
        for k in 1..n {
            g.insert(((1 << k), k), (*distances.get(0, k), 0));
        }
        for subset in MaskSubsetIterator::new(n - 1).skip_while(|m| m.count_ones() < 2) {
            let mask = subset << 1;
            for k in BitmaskIterator::new(mask) {
                let mut best_path = usize::MAX;
                let mut best_p = 0;
                for m in BitmaskIterator::new(mask).filter(|&m| m != k) {
                    let path = g[&((mask & (!(1 << k))), m)].0 + distances.get(m, k);
                    if path < best_path {
                        best_path = path;
                        best_p = m;
                    }
                }
                g.insert(((mask), k), (best_path, best_p));

            }
        }
        let mut opt = usize::MAX;
        let mut last_p = 0;
        let complete_mask = (1usize << n) - 2;
        for k in 1..n {
            let path = g[&(complete_mask, k)].0 + distances.get(k, 0);
            if path < opt {
                opt = g[&(complete_mask, k)].0 + distances.get(k, 0);
                last_p = k
            }
        }
        let mut p_mask = complete_mask;
        let mut prev = last_p;
        let mut tour = Vec::with_capacity(n);
        tour.push(0);
        while p_mask != 0 {
            tour.push(prev);
            let next_p = g[&(p_mask, prev)].1;
            p_mask &= !(1 << prev);
            prev = next_p;
        }
        tour.push(0);
        println!("g.len() = {}", g.len());
        println!("tour = {tour:?}");
        opt
    }
}

pub struct BitmaskIterator {
    bitmask: usize,
    n: usize,
}

impl BitmaskIterator {
    pub fn new(bitmask: usize) -> Self {
        let n = 0;
        Self { bitmask, n }
    }
}

impl Iterator for BitmaskIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitmask == 0 {
            return None;
        }
        while self.bitmask & 1 == 0 {
            self.bitmask >>= 1;
            self.n += 1;
        }
        let res = self.n;
        self.bitmask >>= 1;
        self.n += 1;
        Some(res)
    }
}

pub struct MaskSubsetIterator {
    set: usize,
    sub: usize,
    curr: usize,
}
// https://stackoverflow.com/questions/19299553/power-set-generated-by-bits
impl MaskSubsetIterator {
    pub fn new(n: usize) -> Self {
        let set = (1usize << n) - 1;
        let sub = 0;
        let curr = 0;
        Self { set, sub, curr }
    }
}

impl Iterator for MaskSubsetIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sub > self.set {
            return None;
        }
        let res = self.curr;

        let y = snoob(self.curr, self.set);
        // println!("snoob({:b}, {:b}) = {:b}", self.curr, self.set, y);
        // println!("self.sub = {:b}", self.sub);
        if y <= self.sub {
            self.sub = (self.sub << 1) + 1;
            self.curr = self.sub;
        } else {
            self.curr = y;
        }
        Some(res)
    }
}
// https://stackoverflow.com/questions/19299553/power-set-generated-by-bits
fn snoob(sub: usize, mut set: usize) -> usize {
    let mut tmp = sub.overflowing_sub(1).0;
    let mut rip = set
        & (tmp
            .overflowing_add(
                (sub & (0usize.overflowing_sub(sub).0))
                    .overflowing_sub(set)
                    .0,
            )
            .0);
    let mut sub = (tmp & sub) ^ rip;
    while {
        sub &= sub.overflowing_sub(1).0;
        sub != 0
    } {
        tmp = set & (0usize.overflowing_sub(set).0);
        rip ^= tmp;
        set ^= tmp;
    }
    rip
}
