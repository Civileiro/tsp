use rand::prelude::*;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct City {
    x: usize,
    y: usize,
}

impl City {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn random(max_x: usize, max_y: usize) -> Self {
        Self::new(
            thread_rng().gen_range(0..max_x),
            thread_rng().gen_range(0..max_y),
        )
    }
    pub fn distance(&self, other: &Self) -> usize {
        let abs_x = self.x.abs_diff(other.x);
        let abs_y = self.y.abs_diff(other.y);
        ((abs_x * abs_x + abs_y * abs_y) as f64).sqrt() as usize
    }
}

#[derive(Debug)]
pub struct Tsp {
    pub cities: Vec<City>,
}

impl Tsp {
    pub fn random(size: usize, num_cities: usize) -> Self {
        Self {
            cities: (0..num_cities).map(|_| City::random(size, size)).collect(),
        }
    }

    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        println!("Loading {:?}", path);
        let content = std::fs::read_to_string(path)?;
        let cities = content
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(' ').unwrap();
                City::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<Vec<_>>();
        Ok(Self { cities })
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        if path.exists() {
            panic!("path exists");
        }

        let mut file = std::fs::File::create(path)?;
        println!("Created {:?}", path.canonicalize().unwrap());

        for city in &self.cities {
            file.write_fmt(format_args!("{} {}\n", city.x, city.y))?
        }
        Ok(())
    }

    pub fn solve<S: TspSolution>(&self) -> usize {
        S::solve(self)
    }
}

pub trait TspSolution {
    fn solve(tsp: &Tsp) -> usize;
}
