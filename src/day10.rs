use crate::common::Day;

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Add(i32),
    Noop,
}

pub struct Cpu {
    reg_x: i32,
    cycle: usize,
    program: Vec<Instr>,
    is_adding: Option<i32>,
    pc: usize,
}

static CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

impl Cpu {
    fn with_program(program: &Program) -> Self {
        Cpu {
            reg_x: 1,
            cycle: 1,
            program: program.to_vec(),
            is_adding: None,
            pc: 0,
        }
    }

    fn progress_cycle(&mut self) -> bool {
        self.cycle += 1;
        match self.is_adding {
            Some(n) => {
                self.reg_x += n;
                self.is_adding = None;
                return true;
            }
            None => {
                return self.run_next_instr();
            }
        }
    }

    fn run_next_instr(&mut self) -> bool {
        if self.pc < self.program.len() {
            match self.program[self.pc] {
                Instr::Noop => {}
                Instr::Add(n) => self.is_adding = Some(n),
            };
            self.pc += 1;
            return true;
        } else {
            return false;
        }
    }
}

pub type Program = Vec<Instr>;

struct Crt {
    screen: [bool; 40 * 6],
    cycle: usize,
}

impl Crt {
    fn new() -> Self {
        Crt {
            screen: [false; 40 * 6],
            cycle: 0,
        }
    }

    fn progress_cycle(&mut self, sprite_pos: i32) {
        let col = i32::try_from(self.cycle % 40).unwrap();
        if sprite_pos - 1 == col
            || sprite_pos == col
            || sprite_pos + 1 == i32::try_from(col).unwrap()
        {
            self.screen[self.cycle] = true;
        }
        self.cycle += 1;
    }

    fn render(&self) {
        self.screen.chunks(40).for_each(|x| {
            x.iter()
                .for_each(|&x| if x { print!("X") } else { print!(".") });
            println!();
        });
    }
}

pub struct Day10;

impl<'a> Day<'a> for Day10 {
    type Input = Program;
    type Output = usize;

    fn day_number() -> usize {
        10
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let mut cpu = Cpu::with_program(input);
        let mut result = 0;
        while cpu.progress_cycle() {
            if CYCLES.contains(&cpu.cycle) {
                result += i32::try_from(cpu.cycle).unwrap() * cpu.reg_x;
            }
        }
        result as usize
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut cpu = Cpu::with_program(input);
        let mut crt = Crt::new();
        crt.progress_cycle(cpu.reg_x);
        while cpu.progress_cycle() {
            crt.progress_cycle(cpu.reg_x);
        }
        crt.render();
        0
    }

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|x| {
                if let Some(n) = x.strip_prefix("addx ") {
                    Instr::Add(n.parse::<i32>().unwrap())
                } else if x == "noop" {
                    Instr::Noop
                } else {
                    panic!("Invalid input");
                }
            })
            .collect()
    }
}
