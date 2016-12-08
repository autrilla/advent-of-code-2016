const INSTRUCTIONS: &'static str = "DLRRRRLRLDRRRURRURULRLLULUURRRDDLDULDULLUUDLURLURLLDLUUUDUUUULDRDUUDUDDRRLRDDDUDLDLLRUURDRULUULRLRDULULLRLRLRLDRLUULDLDDDDRRLRUUUDDRURRULLLRURLUURULLRLUDDLDRUULDRURULRRRLLLRDLULDRRDDUDLURURLDULDRDRLDDUURRDUDDRDUURDULDUURDUDRDRULDUDUULRRULUUURDUURUDLDURDLRLURUUDRRDLRUDRULRURLDLLDLLRRDRDRLRRRULDRRLDUURLUUDLUUDDLLRULRDUUDURURLUURDRRRUDLRDULRRRLDRDULRUUDDDLRDUULDRLLDRULUULULRDRUUUULULLRLLLRUURUULRRLDDDRULRRRUDURUR
RULRUUUDLLUDURDRDDLLRLLUDRUDDRLRRDLDLDRDULDLULURDLUDDDUULURLDRUUURURLLRRDDDUUDRLRLLDLDRDDDRDUDLRDRDLLLDDLDUDDRUDUUDLLLLLDULRLURRRLLURUUULUDRLRLRLURRDRLLLRLLULRLLLDDLRLRDLUUUUUDULULDDULLUDUURDLRUDLRUDLRLLRLDLULRLDUDRURURDLRULDLULULDLLDLDLDLLLUDUDDLRLRRDULLUDRDDLLLDUURDULUDURLLLDRUDDDLRLULDLDRRDDDRDULDDUDRDDULLULRRLRUULRDUDURUDULUDUDURLDRDUUDDRRLRURDRRLRDDDDRUDLUDLDDLRDLUUDLRRURDDLURDLRDLLRDRDLDLDUUUURULUULDDDDLDULUURRRULUDLLLDRULDRURL
RRRLRDLLDUURDRRRLURDUULUDURDRRUUDURURRLDLLDRDLRRURDDUDDURLRUUDDULULRUUDRLUUDDLLDDDLRRRDLLLLLLRRURDULDLURRURRDDLDDDUDURRDURRRLUDRRULLRULDRLULRULDDRLLRDLRDUURULURLUURLRRULDULULUULDUDLRLDRDDRRRUUULULDUURLRLLURRLURDUUDDDRUULDLLLDRUURLRRLLDDUDRDLDDDULDRDDDUDRRLLLULURDUDLLUUURRLDULURURDDLUDLLRLDRULULURDLDRLURDLRRDRRUULLULDLURRDDUDRDDDLDUDLDRRUDRULDLDULRLLRRRRDDRLUURRRRDDLLRUURRLRURULDDULRLULRURRUULDUUDURDRRLRLUDRULDRUULUUDRDURDURRLULDDDULDDLRDURRUUUUUDDRRDLRDULUUDDL
DRRLLRRLULDDULRDDLRLDRURDDUDULURRDLUUULURRRLLRLULURLLRLLDLLUDDLLRDRURRDLDDURRURDRDDUDDDLLRLDLDLDDDDRRRRUDUDLRDUDDURLLRURRDUDLRLLUDDRLDUUDDLLLUDRRRLLDDULUDDRLLUDDULLDDLRLDLRURRLUDDLULULDLUURDLLUDUDRRRRDULUDLRRLRUDDUUDRRLLRUUDRRLDDLRRRUDRRDRRDDUDLULLURRUURLLLDRDDLUDDDUDDRURURDLRUULLRDRUUDRDUDRLULLDURUUULDDLDRDRUDRUDUULDDRLRDRRDRRRRLRLRUULDDUUDDLLLLRRRDUDLRDLDUDDUURLUDURLDRRRDRUDUDRLDLRLDRDDLUDRURLRDRDLDUDDDLRLULLUULURLDDDULDUDDDLDRLDLURULLUDLLDRULDLLLDUL
LDULURUULLUDLDDRLLDURRULRLURLLURLRRLRDLDDRUURULLRUURUURRUDDDLRRLDDLULDURLLRDURDLLLURLDRULLURLRLDRDRULURDULDLLDUULLLDUDULDURLUDRULRUUUUUUDUUDDDLLURDLDLRLRDLULRDRULUUDRLULLURLRLDURDRRDUDDDURLLUUDRRURUDLDUDRLRLDRLLLLDLLLURRUDDURLDDRULLRRRRDUULDLUDLDRDUUURLDLLLDLRLRRLDDULLRURRRULDLURLURRRRULUURLLUULRURDURURLRRDULLDULLUDURDUDRLUULULDRRDLLDRDRRULLLDDDRDUDLRDLRDDURRLDUDLLRUDRRRUDRURURRRRDRDDRULRRLLDDRRRLDLULRLRRRUDUDULRDLUDRULRRRRLUULRULRLLRLLURDLUURDULRLDLRLURDUURUULUUDRLLUDRULULULLLLRLDLLLDDDLUULUDLLLDDULRDRULURDLLRRDRLUDRD";

#[derive(Debug, Clone)]
struct Digit(u8);

impl Digit {
    fn up(&self) -> Digit {
        Digit(match self.0 {
            n @ 1 ... 3 => n,
            n => n - 3,
        })
    }

    fn down(&self) -> Digit {
        Digit(match self.0 {
            n @ 7 ... 9 => n,
            n => n + 3,
        })
    }

    fn left(&self) -> Digit {
        Digit(match self.0 {
            n @ 1 | n @ 4 |  n @ 7 => n,
            n => n - 1,
        })
    }

    fn right(&self) -> Digit {
        Digit(match self.0 {
            n @ 3 | n @ 6 | n @ 9 => n,
            n => n + 1,
        })
    }
}

struct SimpleKeypad {
    position: Digit,
}

impl SimpleKeypad {
    fn execute(&mut self, instructions: &str) -> Vec<Digit> {
        instructions.lines().map(|line| {
            line.chars().map(|char| {
                self.position = match char {
                    'U' => self.position.up(),
                    'L' => self.position.left(),
                    'R' => self.position.right(),
                    'D' => self.position.down(),
                    d => panic!("Unknown direction {}", d),
                };
            }).count();
            self.position.clone()
        }).collect()
    }
}

#[derive(Debug, Clone)]
enum ComplexDigit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

struct ComplexKeypad {
    position: ComplexDigit,
}

impl ComplexDigit {
    fn up(&self) -> ComplexDigit {
        use ComplexDigit::*;
        match *self {
            Three => One,
            Six => Two,
            Seven => Three,
            Eight => Four,
            A => Six,
            B => Seven,
            C => Eight,
            D => B,
            ref digit => digit.clone(),
        }
    }

    fn down(&self) -> ComplexDigit {
        use ComplexDigit::*;
        match *self {
            One => Three,
            Two => Six,
            Three => Seven,
            Four => Eight,
            Six => A,
            Seven => B,
            Eight => C,
            B => D,
            ref digit => digit.clone(),
        }
    }

    fn left(&self) -> ComplexDigit {
        use ComplexDigit::*;
        match *self {
            Six => Five,
            Three => Two,
            Seven => Six,
            B => A,
            Four => Three,
            Eight => Seven,
            C => B,
            Nine => Eight,
            ref digit => digit.clone(),
        }
    }

    fn right(&self) -> ComplexDigit {
        use ComplexDigit::*;
        match *self {
            Five => Six,
            Two => Three,
            Six => Seven,
            A => B,
            Three => Four,
            Seven => Eight,
            B => C,
            Eight => Nine,
            ref digit => digit.clone(),
        }
    }
}

impl ComplexKeypad {
    fn execute(&mut self, instructions: &str) -> Vec<ComplexDigit> {
        instructions.lines().map(|line| {
            line.chars().map(|char| {
                self.position = match char {
                    'U' => self.position.up(),
                    'L' => self.position.left(),
                    'R' => self.position.right(),
                    'D' => self.position.down(),
                    d => panic!("Unknown direction {}", d),
                };
            }).count();
            self.position.clone()
        }).collect()
    }
}

fn main() {
    println!("{:?}", SimpleKeypad {
        position: Digit(0),
    }.execute(INSTRUCTIONS));
    println!("{:?}", ComplexKeypad {
            position: ComplexDigit::Five,
    }.execute(INSTRUCTIONS));
}
