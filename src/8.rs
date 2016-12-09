#[macro_use(s)]
extern crate ndarray;
use std::io::prelude::*;
use std::fs::File;
use ndarray::{Array2, Axis};

fn rectangle(array: &mut Array2<u8>, rows: isize, cols: isize) {
    let mut slice = array.slice_mut(s![..rows, ..cols]);
    slice.fill(1);
}

fn rotate_column(array: &mut Array2<u8>, col: isize, amount: usize) {
    let mut slice = array.slice_mut(s![.., col..col+1]);
    let mut slice = slice.subview_mut(Axis(1), 0);
    let old_elements = slice.iter().cloned().enumerate().collect::<Vec<_>>();
    let size = slice.shape()[0];
    for (i, element) in old_elements {
        slice[[(i + amount) % size]] = element;
    }
}

fn rotate_row(array: &mut Array2<u8>, row: isize, amount: usize) {
    let mut slice = array.slice_mut(s![row..row+1, ..]);
    let mut slice = slice.subview_mut(Axis(0), 0);
    let old_elements = slice.iter().cloned().enumerate().collect::<Vec<_>>();
    let size = slice.shape()[0];
    for (i, element) in old_elements {
        slice[[(i + amount) % size]] = element;
    }
}

fn main() {
    let mut s = String::new();
    File::open("src/8.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let mut screen = Array2::<u8>::zeros((6, 50));
    for line in s.lines() {
        let words = line.split(" ").collect::<Vec<_>>();
        match words[0] {
            "rect" => {
                let size = words[1].split("x").collect::<Vec<_>>();
                rectangle(&mut screen, size[1].parse().unwrap(), size[0].parse().unwrap());
            },
            "rotate" => {
                let amount = words.iter().last().unwrap().parse().unwrap();
                let which = words[2].split("=").nth(1).unwrap().parse().unwrap();
                match words[1] {
                    "row" => {
                        rotate_row(&mut screen, which, amount);
                    },
                    "column" => {
                        rotate_column(&mut screen, which, amount);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
        println!("{:?}", screen);
    }
    println!("{}", screen.scalar_sum());
}
