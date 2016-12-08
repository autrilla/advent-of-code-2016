#[macro_use]
extern crate lazy_static;
extern crate itertools;
use itertools::Zip;

use std::io::prelude::*;
use std::fs::File;
use std::iter::{FromIterator, Iterator};

#[derive(Clone)]
struct Triangle(u16, u16, u16);

impl FromIterator<u16> for Triangle {
    fn from_iter<T: IntoIterator<Item = u16>>(iterator: T) -> Self {
        let mut iter = iterator.into_iter();
        Triangle(iter.next().expect("Invalid input format"),
                 iter.next().expect("Invalid input format"),
                 iter.next().expect("Invalid input format"))
    }
}

lazy_static! {
    static ref TRIANGLES: Vec<Triangle> = {
        let mut s = String::new();
        File::open("src/3.txt")
            .expect("Could not open input file")
            .read_to_string(&mut s)
            .expect("Could not read input file");
        s.lines().map(|line| {
            line.trim().split(" ").filter(|w| !w.is_empty()).map(|number| number.parse::<u16>()
                                .expect("Invalid input format")).collect::<Triangle>()
        }).collect()
    };

    static ref TRIANGLES2: Vec<Triangle> = {
        let mut s = String::new();
        File::open("src/3.txt")
            .expect("Could not open input file")
            .read_to_string(&mut s)
            .expect("Could not read input file");
        let numbers = s.lines().flat_map(|line| line.split(" ").filter(|w| !w.is_empty()))
            .map(|w| w.parse::<u16>().expect("Invalid input format"))
            .collect::<Vec<u16>>();
        numbers.chunks(9).flat_map(|chunk| {
            let one = chunk.iter().take(3);
            let two = chunk.iter().skip(3).take(3);
            let three = chunk.iter().skip(6).take(3);
            Zip::new((one, two, three)).into_iter().map(|(one, two, three)| Triangle(*one, *two, *three))
        }).collect()
    };
}

impl Triangle {
    fn is_valid(&self) -> bool {
        !(self.0 + self.1 <= self.2 ||
          self.1 + self.2 <= self.0 ||
          self.0 + self.2 <= self.1)
    }
}
fn main() {
    println!("{}",
             TRIANGLES.iter()
             .filter(|&triangle| triangle.is_valid())
             .count());
    println!("{}",
             TRIANGLES2.iter()
             .filter(|&triangle| triangle.is_valid())
             .count());
}
