mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let funcs = [
        day1::PARTS,
        day2::PARTS,
        day3::PARTS,
        day4::PARTS,
        day5::PARTS,
        day6::PARTS,
        day7::PARTS,
        day8::PARTS,
        day9::PARTS,
        day10::PARTS,
        day11::PARTS,
        day12::PARTS,
        day13::PARTS,
        day14::PARTS,
    ];
    let mut args = std::env::args();
    args.next();
    if let Some(x) = args.next() {
        if let Ok(x) = x.parse::<usize>() {
            if let Some(y) = args.next() {
                if let Ok(y) = y.parse::<usize>() {
                    if let Some(x) = funcs.get(x - 1) {
                        if let Some(x) = x.get(y - 1) {
                            let timer = std::time::Instant::now();
                            x();
                            println!("Took {:?}", timer.elapsed());
                        } else {
                            println!("Not implemented");
                        }
                    } else {
                        println!("Not implemented");
                    }
                } else {
                    println!("Must enter numbers!");
                }
            } else {
                println!("Pass day and part as commandline parameters");
            }
        } else {
            println!("Must enter numbers!");
        }
    } else {
        println!("Pass day and part as commandline parameters");
    }
}
