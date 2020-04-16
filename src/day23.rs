use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

enum Ins {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}

fn compile_input(filename: &str) -> Vec<Ins> {
    let reg = Regex::new(r"\s+|,\s*").unwrap();
    //
    fn register(x: &str) -> usize {
        match x.chars().next() {
            Some(x) => x as usize - b'a' as usize,
            None => unreachable!(),
        }
    }
    //
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(
            |l| match reg.split(&l).collect::<ArrayVec<[_; 3]>>().as_slice() {
                ["hlf", x] => Ins::Hlf(register(x)),
                ["tpl", x] => Ins::Tpl(register(x)),
                ["inc", x] => Ins::Inc(register(x)),
                ["jmp", x] => Ins::Jmp(x.parse().unwrap()),
                ["jie", x, y] => Ins::Jie(register(x), y.parse().unwrap()),
                ["jio", x, y] => Ins::Jio(register(x), y.parse().unwrap()),
                _ => unreachable!(),
            },
        )
        .collect()
}

fn run_program(program: &[Ins], registers: &mut [u64]) {
    let mut i = 0;
    //
    while i >= 0 && i < program.len() as isize {
        match program[i as usize] {
            Ins::Hlf(x) => registers[x] /= 2,
            Ins::Tpl(x) => registers[x] *= 3,
            Ins::Inc(x) => registers[x] += 1,
            Ins::Jmp(x) => i += x - 1,
            Ins::Jie(x, y) => {
                if registers[x] % 2 == 0 {
                    i += y - 1;
                }
            }
            Ins::Jio(x, y) => {
                if registers[x] == 1 {
                    i += y - 1;
                }
            }
        }
        i += 1;
    }
}

fn part1() {
    let program = compile_input("inputfiles/day23/input.txt");
    //
    let mut registers = [0, 0];
    //
    run_program(&program, &mut registers);
    //
    println!("{}", registers[1]);
}

fn part2() {
    let program = compile_input("inputfiles/day23/input.txt");
    //
    let mut registers = [1, 0];
    //
    run_program(&program, &mut registers);
    //
    println!("{}", registers[1]);
}
