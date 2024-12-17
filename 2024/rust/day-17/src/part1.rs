#[derive(Debug)]
enum RegisterField {
    A,
    B,
    C,
}

#[derive(Debug)]
enum Operand {
    Literal(i32),         // 0-3 literal value
    Combo(RegisterField), // 4-A 5-B 6-C 7-reserved
}

impl Operand {
    fn new(op: i32) -> Self {
        match op {
            0..=3 => Operand::Literal(op),
            4 => Operand::Combo(RegisterField::A),
            5 => Operand::Combo(RegisterField::B),
            6 => Operand::Combo(RegisterField::C),
            _ => panic!("Invalid operand"),
        }
    }

    fn get_value(&self, reg: &Register) -> i32 {
        match self {
            Operand::Literal(l) => *l,
            Operand::Combo(RegisterField::A) => reg.a,
            Operand::Combo(RegisterField::B) => reg.b,
            Operand::Combo(RegisterField::C) => reg.c,
        }
    }
}

enum Instruction {
    Adv, // opcode 0(division): A <- division 2 ^ (combo operand) / A
    Bxl, // opcode 1(XOR): B <- B ^ (literal operand)
    Bst, // opcode 2: B <- (combo operand) % 8
    Jnz, // opcode 3: if A == 0 pass else jump to (literal operand) (ic not increased after jump)
    Bxc, // opcode 4(XOR): B <- B ^ C
    Out, // opcode 5: print (combo operand) % 8
    Bdv, // opcode 6: same as Adv but save result in B
    Cdv, // opcode 7: same as Adv but save result in C
}

impl Instruction {
    fn new(ins: i32) -> Self {
        match ins {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug)]
struct Register {
    a: i32,
    b: i32,
    c: i32,
}

impl Register {
    fn new(a: i32, b: i32, c: i32) -> Register {
        Register { a, b, c }
    }

    fn process(&mut self, instructions: Vec<(i32, i32)>) -> String {
        println!("start: {:#?}", self);
        let mut result: Vec<i32> = Vec::new();
        let mut i = 0;
        while i < instructions.len() {
            let (ins, op) = instructions[i];
            let ins = Instruction::new(ins);
            let op = Operand::new(op);

            match ins {
                Instruction::Adv => self.execute_div(op, RegisterField::A),
                Instruction::Bxl => {
                    self.b ^= op.get_value(self);
                }
                Instruction::Bst => {
                    self.b = op.get_value(self) % 8;
                }
                Instruction::Jnz => {
                    if self.a != 0 {
                        i = op.get_value(self) as usize;
                        continue;
                    }
                }
                Instruction::Bxc => self.b ^= self.c,
                Instruction::Out => result.push(op.get_value(self) % 8),
                Instruction::Bdv => self.execute_div(op, RegisterField::B),
                Instruction::Cdv => self.execute_div(op, RegisterField::C),
            }
            println!("{:#?}", self);
            i += 1;
        }

        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn execute_div(&mut self, op: Operand, target: RegisterField) {
        match target {
            RegisterField::A => {
                let value = op.get_value(self);
                self.a = self.a / 2i32.pow(value as u32);
            }
            RegisterField::B => {
                let value = op.get_value(self);
                self.b = self.b / 2i32.pow(value as u32);
            }
            RegisterField::C => {
                let value = op.get_value(self);
                self.c = self.c / 2i32.pow(value as u32);
            }
        }
    }
}

pub fn solve(input: &str) -> String {
    let input: Vec<&str> = input.trim().split("\n\n").collect();
    let registers: Vec<i32> = input[0]
        .split("\n")
        .map(|x| {
            x.split(" ").collect::<Vec<&str>>()[2]
                .parse::<i32>()
                .unwrap()
        })
        .collect();

    let mut reg = Register::new(registers[0], registers[1], registers[2]);

    let instructions: Vec<(i32, i32)> = input[1].split(" ").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .collect();

    reg.process(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(solve(input), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
