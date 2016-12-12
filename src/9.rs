#[macro_use]
extern crate nom;

use nom::IResult;
use std::str::FromStr;

named!(mark<(usize, usize)>,
    do_parse!(
        tag!("(") >>
        chars: map_res!(
                map_res!(
                    nom::digit,
                    std::str::from_utf8
                ),
            std::str::FromStr::from_str
        ) >>
        tag!("x") >>
        times: map_res!(
            map_res!(
                nom::digit,
                std::str::from_utf8
            ),
            std::str::FromStr::from_str
        ) >>
        tag!(")") >>
        (chars, times)
    )
);

fn not_open_parenthesis(c: u8) -> bool { c != b'(' }


named!(expand<String>,
       alt!(
           map_res!(
               take_while1!(not_open_parenthesis),
               |x: &[u8]| String::from_utf8(x.to_vec())
           ) |
           map_res!(
               do_parse!(
                   m: mark >>
                   s: take!(m.0) >>
                   (s, m.1)
               ),
               |(s, r): (&[u8], usize)| {
                   let mut v : Vec<_> = Vec::new();
                   for i in std::iter::repeat(s).take(r) {
                       v.extend(i);
                   }
                   String::from_utf8(v)
               }
         )
    )
);

named!(expand_once<String>,
       fold_many1!(expand, String::new(), |mut acc, s: String| { acc += s.as_str(); acc })
);

fn main() {
    let lines = include_str!("9.txt");
    let input = lines.as_bytes();
    println!("{:?}", expand_once(input).unwrap().1.trim().len());
    let mut lines = lines.to_owned();
    while lines.contains('(') {
        lines = {
            let bytes = lines.as_bytes();
            let processed = expand_once(bytes).unwrap();
            processed.1
        };
        println!("{:?}", lines.trim().len());
    }
}
