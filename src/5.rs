extern crate rustc_serialize;
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;

fn main() {
    let mut hasher = Md5::new();

    let key = "ugkcyxxp".as_bytes();
    let mut first_password = "".to_owned();
    let second_password : &mut [Option<char>] = &mut [None; 8];
    for i in 0.. {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        let output = output.to_hex();
        if output.starts_with("00000") {
            let index = output.clone().chars().nth(5).unwrap();
            let character = output.clone().chars().nth(6).unwrap();
            if first_password.len() < 8 {
                first_password.push(index);
            }
            let index = index.to_digit(16).unwrap() as usize;
            if index < 8 {
                if second_password[index].is_none() {
                    second_password[index] = Some(character);
                }
            }
            if first_password.len() == 8 && second_password.iter().filter(|&x| x.is_none()).count() == 0 {
                let second_password = second_password.iter().map(|x| x.unwrap()).collect::<String>();
                println!("First password: {}\nSecond password: {:?}",
                         first_password,
                         second_password);
                break;
            }
        }
        hasher.reset();
    }
}
