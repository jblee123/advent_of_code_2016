use std::fmt;
use std::fs;
use std::str::FromStr;

type Literal = i32;
type Register = usize;

#[derive(Clone, Copy)]
enum Operand {
    Lit(Literal),
    Reg(Register),
}

fn reg_num_to_char(reg: Register) -> char {
    ((reg as u8) + ('a' as u8)) as char
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Lit(lit) => write!(f, "{}", lit),
            Operand::Reg(reg) => write!(f, "{}", reg_num_to_char(reg)),
        }
    }
}

enum Instr {
    Cpy(Operand, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Operand, Literal),
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instr::Cpy(op, reg) =>
                write!(f, "cpy {} {}", op, reg_num_to_char(reg)),
            Instr::Inc(reg) => write!(f, "inc {}", reg_num_to_char(reg)),
            Instr::Dec(reg) => write!(f, "dec {}", reg_num_to_char(reg)),
            Instr::Jnz(op, lit) => write!(f, "jnz {} {}", op, lit,),
        }
    }
}

impl Instr {
    pub fn from_str(instr_str: &str) -> Instr {
        let parts = instr_str.split(" ").collect::<Vec<&str>>();

        let assert_for_line = |test: bool| {
            if !test {
                panic!("bad line: {}", instr_str);
            }
        };

        let ensure_line_len = |required_len: usize| {
            assert_for_line(parts.len() == required_len);
        };

        let is_register = |operand_str: &str| -> bool {
            if operand_str.len() != 1 {
                return false;
            }

            match operand_str.chars().nth(0).unwrap() {
                'a'..='d' => true,
                _ => false,
            }
        };

        let ensure_register = |operand_str: &str| {
            assert_for_line(is_register(operand_str));
        };

        let to_register = |operand_str: &str| -> Register {
            ensure_register(operand_str);
            (operand_str.bytes().nth(0).unwrap() - ('a' as u8)) as Register
        };

        let get_register_or_literal = |operand_str: &str| -> Operand {
            if is_register(operand_str) {
                Operand::Reg(to_register(operand_str))
            } else {
                Operand::Lit(Literal::from_str(parts[1]).unwrap())
            }
        };

        if parts.is_empty() {
            panic!("bad line: {}", instr_str);
        }

        match parts[0] {
            "cpy" => {
                ensure_line_len(3);
                let op1 = get_register_or_literal(parts[1]);
                let op2 = to_register(parts[2]);
                Instr::Cpy(op1, op2)
            },
            "inc" => {
                ensure_line_len(2);
                Instr::Inc(to_register(parts[1]))
            },
            "dec" => {
                ensure_line_len(2);
                Instr::Dec(to_register(parts[1]))
            },
            "jnz" => {
                ensure_line_len(3);
                let op1 = get_register_or_literal(parts[1]);
                let op2 = Literal::from_str(parts[2]).unwrap();
                Instr::Jnz(op1, op2)
            },
            _ => panic!("bad op: {}", parts[0]),
        }
    }
}

const NUM_REGS: usize = 4;
struct Cpu {
    regs: [i32; NUM_REGS],
    instr_ptr: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0; NUM_REGS],
            instr_ptr: 0,
        }
    }

    fn exec(&mut self, instr: &Vec::<Instr>) -> bool {
        if self.instr_ptr >= instr.len() {
            return false;
        }

        match instr[self.instr_ptr] {
            Instr::Cpy(src, tgt_reg) => {
                self.regs[tgt_reg] = match src {
                    Operand::Lit(lit) => lit,
                    Operand::Reg(src_reg) => self.regs[src_reg],
                }
            },
            Instr::Inc(reg) => {
                self.regs[reg] += 1;
            },
            Instr::Dec(reg) => {
                self.regs[reg] -= 1;
            },
            Instr::Jnz(test, offset) => {
                let do_jmp = match test {
                    Operand::Lit(lit) => lit,
                    Operand::Reg(reg) => self.regs[reg],
                };
                if do_jmp != 0 {
                    self.instr_ptr = self.instr_ptr.wrapping_add(
                        (offset - 1) as usize);
                }
            },
        };

        self.instr_ptr += 1;
        self.instr_ptr < instr.len()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = input.lines();

    let mut instrs = Vec::<Instr>::new();

    for line in lines {
        instrs.push(Instr::from_str(line));
    }

    let mut cpu = Cpu::new();
    while cpu.exec(&instrs) {
    }

    println!("reg a: {}", cpu.regs[0]);
}
