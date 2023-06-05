use std::collections::HashMap;

use crate::{
    grid::Grid,
    tsp::{Tsp, TspSolution},
};

pub struct HeldKarpRecursivo {
    distances: Grid<usize>,
    memo: HashMap<(usize, usize), usize>,
}

impl TspSolution for HeldKarpRecursivo {
    fn solve(tsp: &Tsp) -> usize {
        Self::new(tsp).solution()
    }
}

impl HeldKarpRecursivo {
    fn new(tsp: &Tsp) -> Self {
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
        let memo = HashMap::<(usize, usize), usize>::new();
        Self { distances, memo }
    }
    fn solution(&mut self) -> usize {
        let mut ans = usize::MAX;
        let n = self.distances.height;
        for i in 1..n {
            ans = ans.min(self.min_travel_from_zero(i, (1 << n) - 1) + self.distances.get(i, 0));
        }
        ans
    }
    pub fn min_travel_from_zero(&mut self, i: usize, mask: usize) -> usize {
        if mask == (1 << i) | 1 {
            return *self.distances.get(0, i);
        }
        if let Some(memo_res) = self.memo.get(&(i, mask)) {
            return *memo_res;
        };

        let mut res = usize::MAX;
        for j in 0..self.distances.height {
            if mask & (1 << j) != 0 && j != i && j != 0 {
                let min_travel_to_end = self.min_travel_from_zero(j, mask & (!(1 << i)));
                let end_to_begin = self.distances.get(j, i);

                res = res.min(min_travel_to_end + end_to_begin)
            }
        }
        self.memo.insert((i, mask), res);
        res
    }
}
