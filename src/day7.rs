use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use lazy_static::*;

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG1: Regex = Regex::new(r"(.+) -> (\w+)").unwrap();
}

fn eval_wire(
    program: &HashMap<String, Vec<String>>,
    memory: &mut HashMap<String, u16>,
    wire: &str,
) -> u16 {
    if let Some(val) = memory.get(wire) {
        *val
    } else {
        let val = match program[wire].iter().map(|s| &s[..]).collect::<Vec<_>>()
            [..]
        {
            [a] => {
                if let Ok(n) = a.parse::<u16>() {
                    n
                } else {
                    eval_wire(program, memory, a)
                }
            }
            ["NOT", a] => !eval_wire(program, memory, a),
            [a, "AND", b] => {
                if let Ok(n) = a.parse::<u16>() {
                    n & eval_wire(program, memory, b)
                } else {
                    eval_wire(program, memory, a)
                        & eval_wire(program, memory, b)
                }
            }
            [a, "OR", b] => {
                if let Ok(n) = a.parse::<u16>() {
                    n | eval_wire(program, memory, b)
                } else {
                    eval_wire(program, memory, a)
                        | eval_wire(program, memory, b)
                }
            }
            [a, "LSHIFT", b] => {
                eval_wire(program, memory, a) << b.parse::<u16>().unwrap()
            }
            [a, "RSHIFT", b] => {
                eval_wire(program, memory, a) >> b.parse::<u16>().unwrap()
            }
            _ => panic!("Illegal instruction!"),
        };
        memory.insert(wire.to_owned(), val);
        val
    }
}

fn load_program(filename: &str) -> HashMap<String, Vec<String>> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut s = l.split(" -> ");
            let input = if let Some(input) = s.next() {
                input
                    .split(' ')
                    .map(|s| String::from(s))
                    .collect::<Vec<_>>()
            } else {
                panic!("Line did not match: {}", l)
            };
            let output = s.next().unwrap().to_owned();
            //
            (output, input)
        })
        .collect()
}

fn part1() {
    let program = load_program("inputfiles/day7/input.txt");
    //
    let ans = eval_wire(&program, &mut HashMap::new(), "a");
    println!("a = {}", ans);
}

fn part2() {
    let program = load_program("inputfiles/day7/input.txt");
    //
    let mut memory = HashMap::new();
    let a = eval_wire(&program, &mut memory, "a");
    //
    memory.clear();
    memory.insert("b".to_owned(), a);
    //
    let ans = eval_wire(&program, &mut memory, "a");
    //
    println!("a = {}", ans);
}
