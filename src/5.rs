extern crate rustc_serialize;
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::process;

static THREAD_COUNT: usize = 8;
static CHUNK_SIZE: usize = 100_000;
static KEY: &'static [u8] = &[117, 103, 107, 99, 121, 120, 120, 112];


fn main() {
    let barrier = Arc::new(Barrier::new(THREAD_COUNT + 1));
    let results: Arc<Mutex<Vec<(usize, char, char)>>> = Arc::new(Mutex::new(Vec::new()));
    for thread_number in 0..THREAD_COUNT {
        let results = results.clone();
        let b = barrier.clone();
        thread::spawn(move || {
            let mut i = thread_number * CHUNK_SIZE;
            let mut hasher = Md5::new();
            let mut output = &mut [0; 16];
            loop {
                for j in i..i + CHUNK_SIZE {
                    hasher.input(KEY);
                    hasher.input(j.to_string().as_bytes());
                    hasher.result(output);
                    if output.starts_with(&[0, 0]) && output[2] < 16 {
                        let mut results = results.lock().unwrap();
                        let result = output.to_hex();
                        let index = result.chars().nth(5).unwrap();
                        let character = result.chars().nth(6).unwrap();
                        results.push((j, index, character));
                    }
                    hasher.reset();
                }
                i += THREAD_COUNT * CHUNK_SIZE;
                b.wait();
            }
        });
    }

    let mut first_password = "".to_owned();
    let second_password : &mut [Option<char>] = &mut [None; 8];
    let mut second_password_found_chars = 0;
    loop {
        barrier.wait();
        let mut results = results.lock().unwrap();
        let mut results: Vec<_> = results.drain(0..).collect();
        results.sort_by_key(|x| x.0);
        results.iter().map(|&(_, index, character)| {
            if first_password.len() < 8 {
                first_password.push(index);
            }
            let index = index.to_digit(16).unwrap() as usize;
            if index < 8 {
                if second_password[index].is_none() {
                    second_password[index] = Some(character);
                    second_password_found_chars += 1;
                }
            }
            if second_password_found_chars == second_password.len() {
                let second_password = second_password.iter().map(|x| x.unwrap()).collect::<String>();
                println!("First password: {}\nSecond password: {:?}",
                         first_password,
                         second_password);
                process::exit(0);
            }
        }).count();
    }
}
