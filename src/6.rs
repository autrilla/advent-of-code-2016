#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

lazy_static! { static ref ROWS: Vec<Vec<char>> = {
    let mut s = String::new();
    File::open("src/6.txt")
        .expect("Could not open input file")
        .read_to_string(&mut s)
        .expect("Could not read input file");
    s.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
        }).collect()
    };
}


fn main() {
    let mut cols : Vec<HashMap<char, usize>> = Vec::new();
    for _ in 0..ROWS[0].len() {
        cols.push(HashMap::new());
    }
    for row in ROWS.iter() {
        for (i, c) in row.iter().enumerate() {
            *cols[i].entry(*c).or_insert(0) += 1;
        }
    }

    println!("{}", cols.iter().map(|map| {
        let mut entries = map.iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| a.1.cmp(b.1).reverse());
        entries.iter().map(|&(&c, &i)| c).next().unwrap()
    }).collect::<String>());

    println!("{}", cols.iter().map(|map| {
        let mut entries = map.iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| a.1.cmp(b.1));
        entries.iter().map(|&(&c, &i)| c).next().unwrap()
    }).collect::<String>());
}
