extern crate itertools;

use std::io::prelude::*;
use std::fs::File;
use itertools::Zip;

fn contains_four_letter_palindrome(s: &str) -> bool {
    let iter = Zip::new((s.chars(), s.chars().skip(1), s.chars().skip(2), s.chars().skip(3)));
    for (c0, c1, c2, c3) in iter {
        if c0 == c3 && c1 == c2 && c1 != c0 {
            return true;
        }
    }
    false
}

fn opposite_three_letter_palindromes(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let iter = Zip::new((s.chars(), s.chars().skip(1), s.chars().skip(2)));
    for (c0, c1, c2) in iter {
        if c0 == c2 && c1 != c0 {
            let opposite : String = vec![c1, c0, c1].into_iter().collect();
            out.push(opposite);
        }
    }
    out
}

fn supports_tls(pre_brackets: &str, in_brackets: &str, post_brackets: &str) -> bool {
    !contains_four_letter_palindrome(in_brackets) &&
    (contains_four_letter_palindrome(pre_brackets) ||
     contains_four_letter_palindrome(post_brackets))
}

enum Sequence {
    Supernet(String),
    Hypernet(String),
}

struct IPv7Addr {
    sequences: Vec<Sequence>,
}

impl IPv7Addr {
    fn new(s: &str) -> Self {
        let mut sequences = Vec::new();
        for (i, seq) in s.split(|c| c == '[' || c == ']').enumerate() {
            if i % 2 == 0 {
                sequences.push(Sequence::Supernet(seq.to_owned()));
            } else {
                sequences.push(Sequence::Hypernet(seq.to_owned()));
            }
        }
        IPv7Addr{
            sequences: sequences,
        }
    }

    fn supports_tls(&self) -> bool {
        let mut palindrome_outside = false;
        let mut palindrome_inside = false;
        for seq in self.sequences.iter() {
            match *seq {
                Sequence::Supernet(ref seq) => {
                    if palindrome_outside { continue }
                    if contains_four_letter_palindrome(&seq) {
                        palindrome_outside = true;
                    }
                }
                Sequence::Hypernet(ref seq) => {
                    if palindrome_inside { continue }
                    if contains_four_letter_palindrome(&seq) {
                        palindrome_inside = true;
                    }
                }
            }
        }
        palindrome_outside && !palindrome_inside
    }

    fn supports_ssl(&self) -> bool {
        let mut needles = Vec::new();
        for seq in self.sequences.iter() {
            match *seq {
                Sequence::Supernet(ref seq) => {
                    needles.extend(opposite_three_letter_palindromes(seq));
                }
                _ => (),
            }
        }
        for seq in self.sequences.iter() {
            match *seq {
                Sequence::Hypernet(ref seq) => {
                    for needle in needles.iter() {
                        if seq.contains(needle) {
                            return true;
                        }
                    }
                }
                _ => (),
            }
        }
        false
    }
}

fn main() {
    let mut s = String::new();
    File::open("src/7.txt")
        .expect("Could not open input file")
        .read_to_string(&mut s)
        .expect("Could not read input file");
    let ips: Vec<IPv7Addr> = s.lines().map(|line| IPv7Addr::new(line)).collect();
    println!("{}",
             ips.iter()
             .map(|ip| ip.supports_tls())
             .filter(|x| *x)
             .count());
    println!("{}",
             ips.iter()
             .map(|ip| ip.supports_ssl())
             .filter(|x| *x)
             .count());
}
