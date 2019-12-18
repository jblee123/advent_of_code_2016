use std::fmt;
use std::fs;
use std::str::FromStr;

type Literal = i32;
type Register = usize;

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy)]
enum Instr {
    Cpy(Operand, Operand),
    Inc(Register),
    Dec(Register),
    Jnz(Operand, Operand),
    Tgl(Register),
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instr::Cpy(op1, op2) => write!(f, "cpy {} {}", op1, op2),
            Instr::Inc(reg) => write!(f, "inc {}", reg_num_to_char(reg)),
            Instr::Dec(reg) => write!(f, "dec {}", reg_num_to_char(reg)),
            Instr::Jnz(op1, op2) => write!(f, "jnz {} {}", op1, op2),
            Instr::Tgl(reg) => write!(f, "tgl {}", reg_num_to_char(reg)),
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
                Operand::Lit(Literal::from_str(operand_str).unwrap())
            }
        };

        if parts.is_empty() {
            panic!("bad line: {}", instr_str);
        }

        match parts[0] {
            "cpy" => {
                ensure_line_len(3);
                let op1 = get_register_or_literal(parts[1]);
                let op2 = get_register_or_literal(parts[2]);
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
                let op2 = get_register_or_literal(parts[2]);
                Instr::Jnz(op1, op2)
            },
            "tgl" => {
                ensure_line_len(2);
                Instr::Tgl(to_register(parts[1]))
            },
            _ => panic!("bad op: {}", parts[0]),
        }
    }
}

const NUM_REGS: usize = 4;
#[derive(Debug)]
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

    fn exec(&mut self, instr: &mut Vec::<Instr>) -> bool {
        if self.instr_ptr >= instr.len() {
            return false;
        }

        let val_from_operand = |op| {
            match op {
                Operand::Lit(lit) => lit,
                Operand::Reg(reg) => self.regs[reg],
            }
        };

        match instr[self.instr_ptr] {
            Instr::Cpy(src, tgt) => {
                match tgt {
                    Operand::Reg(tgt_reg) =>
                        self.regs[tgt_reg] = val_from_operand(src),
                    Operand::Lit(_) => {},
                }
            },
            Instr::Inc(reg) => {
                self.regs[reg] += 1;
            },
            Instr::Dec(reg) => {
                self.regs[reg] -= 1;
            },
            Instr::Jnz(test, offset) => {
                let do_jmp = val_from_operand(test) != 0;
                if do_jmp {
                    let offset_val = val_from_operand(offset);
                    self.instr_ptr = self.instr_ptr.wrapping_add(
                        (offset_val - 1) as usize);
                }
            },
            Instr::Tgl(reg) => {
                let target_instr = self.instr_ptr as i32 + self.regs[reg];
                if target_instr >= 0 && target_instr < instr.len() as i32 {
                    let target_instr = target_instr as usize;
                    instr[target_instr] = match instr[target_instr] {
                        Instr::Cpy(src, tgt) => Instr::Jnz(src, tgt),
                        Instr::Inc(reg) => Instr::Dec(reg),
                        Instr::Dec(reg) => Instr::Inc(reg),
                        Instr::Jnz(test, offset) => Instr::Cpy(test, offset),
                        Instr::Tgl(reg) => Instr::Inc(reg),
                    };
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

    let mut instrs: Vec<Instr> = input.lines()
        .map(|line| {
            Instr::from_str(line)
        })
        .collect();

    let mut cpu = Cpu::new();
    cpu.regs[0] = 7; // reg A start val

    while cpu.exec(&mut instrs) {
    }

    println!("reg a: {}", cpu.regs[0]);
}
