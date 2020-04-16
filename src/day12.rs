use std::{fs::File, io::BufReader};

use serde_json::Value;

pub const PARTS: [fn(); 2] = [part1, part2];

fn sum_vals(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| sum_vals(v)).sum(),
        Value::Object(m) => m.iter().map(|(_, v)| sum_vals(v)).sum(),
        _ => 0,
    }
}

fn sum_red(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| sum_red(v)).sum(),
        Value::Object(m) => {
            if !m
                .values()
                .any(|v| matches!(v, Value::String(s) if s == "red"))
            {
                m.iter().map(|(_, v)| sum_red(v)).sum()
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn part1() {
    let ans = sum_vals(
        &serde_json::from_reader(BufReader::new(
            File::open("inputfiles/day12/input.json").unwrap(),
        ))
        .unwrap(),
    );
    //
    println!("{}", ans);
}

fn part2() {
    let ans = sum_red(
        &serde_json::from_reader(BufReader::new(
            File::open("inputfiles/day12/input.json").unwrap(),
        ))
        .unwrap(),
    );
    //
    println!("{}", ans);
}
