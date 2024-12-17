enum Operand {
    Literal(i32), // 0-3 literal value
    Combo(i32),   // 4-A 5-B 6-C 7-reserved
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
            println!("ins: {} op: {}", ins, op);
            // for (ins, op) in instructions {
            let ins = Instruction::new(ins);
            match ins {
                Instruction::Adv => {
                    // A <- division 2 ^ (combo operand) / A
                    match op {
                        1..=3 => self.a = self.a / 2i32.pow(op as u32),
                        4 => self.a = self.a / 2i32.pow(self.a as u32),
                        5 => self.a = self.a / 2i32.pow(self.b as u32),
                        6 => self.a = self.a / 2i32.pow(self.c as u32),
                        _ => panic!("Invalid operand"),
                    }
                }
                Instruction::Bxl => {
                    // B <- B ^ (literal operand)
                    self.b = self.b ^ op;
                }
                Instruction::Bst => {
                    // B <- (combo operand) % 8
                    match op {
                        1..=3 => self.b = op % 8,
                        4 => self.b = self.a % 8,
                        5 => self.b = self.b % 8,
                        6 => self.b = self.c % 8,
                        _ => panic!("Invalid operand"),
                    }
                }
                Instruction::Jnz => {
                    // if A == 0 pass else jump to (literal operand) (ic not increased after jump)
                    if self.a != 0 {
                        i = op as usize;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    // B <- B ^ C
                    self.b = self.b ^ self.c;
                }
                Instruction::Out => {
                    // print (combo operand) % 8
                    match op {
                        1..=3 => result.push(op % 8),
                        4 => result.push(self.a % 8),
                        5 => result.push(self.b % 8),
                        6 => result.push(self.c % 8),
                        _ => panic!("Invalid operand"),
                    }
                }
                Instruction::Bdv => {
                    // same as Adv but save result in B
                    match op {
                        1..=3 => self.b = self.a / 2i32.pow(op as u32),
                        4 => self.b = self.a / 2i32.pow(self.a as u32),
                        5 => self.b = self.a / 2i32.pow(self.b as u32),
                        6 => self.b = self.a / 2i32.pow(self.c as u32),
                        _ => panic!("Invalid operand"),
                    }
                }
                Instruction::Cdv => {
                    // same as Adv but save result in C
                    match op {
                        1..=3 => self.c = self.a / 2i32.pow(op as u32),
                        4 => self.c = self.a / 2i32.pow(self.a as u32),
                        5 => self.c = self.a / 2i32.pow(self.b as u32),
                        6 => self.c = self.a / 2i32.pow(self.c as u32),
                        _ => panic!("Invalid operand"),
                    }
                }
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
