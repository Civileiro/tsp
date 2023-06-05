use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub grid_raw: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, value: &T) -> Self {
        let grid_raw = vec![*value; width * height];
        assert_eq!(width * height, grid_raw.len());
        Self {
            width,
            height,
            grid_raw,
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &self.grid_raw[y * self.width + x]
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &mut self.grid_raw[y * self.width + x]
    }
}

pub struct ColumnIter<'a, T> {
    grid: &'a Grid<T>,
    column: usize,
    front: usize,
    back: usize,
}

impl<'a, T> Debug for ColumnIter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColumnIter")
            .field("column", &self.column)
            .field("front", &self.front)
            .field("back", &self.back)
            .finish()
    }
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let elem = self.grid.get(self.column, self.front);
        self.front += 1;
        Some(elem)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.back - self.front;
        (size, Some(size))
    }
}

impl<'a, T> DoubleEndedIterator for ColumnIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        let elem = self.grid.get(self.column, self.back);
        Some(elem)
    }
}

impl<'a, T> ExactSizeIterator for ColumnIter<'a, T> {}
