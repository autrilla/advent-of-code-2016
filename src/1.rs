use std::collections::HashSet;

static INSTRUCTIONS: &'static str = "R2, L5, L4, L5, R4, R1, L4, R5, R3, R1, L1, L1, R4, L4, L1, R4, L4, R4, L3, R5, R4, R1, R3, L1, L1, R1, L2, R5, L4, L3, R1, L2, L2, R192, L3, R5, R48, R5, L2, R76, R4, R2, R1, L1, L5, L1, R185, L5, L1, R5, L4, R1, R3, L4, L3, R1, L5, R4, L4, R4, R5, L3, L1, L2, L4, L3, L4, R2, R2, L3, L5, R2, R5, L1, R1, L3, L5, L3, R4, L4, R3, L1, R5, L3, R2, R4, R2, L1, R3, L1, L3, L5, R4, R5, R2, R2, L5, L3, L1, L1, L5, L2, L3, R3, R3, L3, L4, L5, R2, L1, R1, R3, R4, L2, R1, L1, R3, R3, L4, L2, R5, R5, L1, R4, L5, L5, R1, L5, R4, R2, L1, L4, R1, L1, L1, L5, R3, R4, L2, R1, R2, R1, R1, R3, L5, R1, R4";

#[derive(Debug)]
enum Instruction {
    Left(i16),
    Right(i16),
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Me {
    direction: Direction,
    position: (i16, i16),
    visited_positions: HashSet<(i16, i16)>,
}

impl Me {
    fn turn_right(&mut self) {
        use Direction::*;
        self.direction = match self.direction {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }

    fn turn_left(&mut self) {
        use Direction::*;
        self.direction = match self.direction {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }

    fn go_forward(&mut self, n: i16) {
        use Direction::*;
        let pos_d = match self.direction {
            North => (1, 0),
            South => (-1, 0),
            East => (0, -1),
            West => (0, 1),
        };
        for _ in 0..n {
            self.visited_positions.insert(self.position);
            self.position = (self.position.0 - pos_d.0, self.position.1 - pos_d.1);
            if self.visited_positions.contains(&self.position) {
                println!("Position: {:?} visited twice", self.position);
            }
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Right(amount) => {
                self.turn_right();
                self.go_forward(amount);
            }
            Instruction::Left(amount) => {
                self.turn_left();
                self.go_forward(amount);
            }
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = INSTRUCTIONS.split(", ")
        .map(|s| {
            let amount = s.split_at(1).1.parse::<i16>().unwrap();
            if s.starts_with("R") {
                Instruction::Right(amount)
            } else {
                Instruction::Left(amount)
            }
        })
        .collect();
    let mut me = Me {
        direction: Direction::North,
        position: (0, 0),
        visited_positions: HashSet::new(),
    };
    for instruction in instructions {
        me.execute(instruction);
    }
    println!("{}", me.position.0.abs() + me.position.1.abs());
}
