#![feature(try_from)]
#![feature(static_in_const)]

#[macro_use]
extern crate lazy_static;

use std::convert::TryFrom;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Room {
    encrypted_name: Vec<String>,
    sector: u16,
    checksum: Vec<char>,
}

impl TryFrom<String> for Room {
    type Err = String;
    fn try_from(s: String) -> Result<Self, Self::Err> {
        let (namesector, checksum) = s.split_at(s.find("[").ok_or("Invalid format")?);
        let checksum: Vec<char> = checksum.chars().skip(1).take_while(|&x| x != ']').collect();
        let namesector: Vec<String> = namesector.split("-").map(str::to_owned).collect();
        let (sector, name) = namesector.split_last().ok_or("Invalid format")?;
        Ok(Room {
            encrypted_name: name.to_vec(),
            sector: sector.parse().map_err(|_| "Invalid format")?,
            checksum: checksum,
        })
    }
}

lazy_static! { static ref ROOMS: Vec<Room> = {
        let mut s = String::new();
        File::open("src/4.txt")
            .expect("Could not open input file")
            .read_to_string(&mut s)
            .expect("Could not read input file");
        s.lines().map(|line| {
            Room::try_from(line.to_owned()).expect("Invalid input format")
        }).collect()
    };
}

impl Room {
    fn is_valid(&self) -> bool {
        let name = self.encrypted_name.join("");
        let mut map : HashMap<char, u16> = HashMap::new();
        for char in name.chars() {
            *map.entry(char).or_insert(0) += 1;
        };
        let mut entries: Vec<(&char, &u16)> = map.iter().collect();
        entries.sort_by(|a, b| {
            match a.1.cmp(b.1).reverse() {
                Ordering::Equal => a.0.cmp(b.0),
                other => other,
            }
        });
        entries.truncate(self.checksum.len());
        let entries : Vec<_> = entries.iter().map(|&(&x, _)| x).collect();
        entries == self.checksum
    }

    fn decrypt(&self) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
        self.encrypted_name.iter().map(|word| {
            word.chars().map(|char| {
                let start_pos = alphabet.binary_search(&char).unwrap();
                let pos = (start_pos + self.sector as usize) % alphabet.len();
                alphabet[pos]
            }).collect::<String>()
        }).collect::<Vec<_>>().join(" ")
    }
}

fn main() {
    let valid_rooms : Vec<_> = ROOMS.iter().cloned().filter(Room::is_valid).collect();
    println!("Sum of sector IDs for valid rooms: {}", valid_rooms.iter().map(|r| r.sector as u64).sum::<u64>());
    let north_pole_room : Vec<_> = valid_rooms.iter().map(|r| (r, r.decrypt()))
        .filter(|&(_, ref name)| name.contains("northpole"))
        .map(|(r, _)| r).collect();
    println!("North pole room: {:?}", north_pole_room);
}
