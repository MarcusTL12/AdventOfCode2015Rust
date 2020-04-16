use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> Vec<Vec<bool>> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Inputfail"),
                })
                .collect()
        })
        .collect()
}

fn _render_lights(lights: &Vec<Vec<bool>>) {
    let h = lights.len();
    let w = lights[0].len();
    for i in 0..h {
        for j in 0..w {
            print!("{}", if lights[i][j] { '#' } else { '.' });
        }
        println!();
    }
}

fn next_lights(current: &Vec<Vec<bool>>, target: &mut Vec<Vec<bool>>) {
    const DIRS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    //
    let h = current.len();
    let w = current[0].len();
    for i in 0..h {
        for j in 0..w {
            let neighbours = DIRS
                .iter()
                .map(|(y, x)| (i as i32 + y, j as i32 + x))
                .filter_map(|(y, x)| {
                    current.get(y as usize).and_then(|row| row.get(x as usize))
                })
                .filter(|&&x| x)
                .count();
            //
            target[i][j] = if current[i][j] {
                matches!(neighbours, 2 | 3)
            } else {
                neighbours == 3
            }
        }
    }
}

fn part1() {
    let mut inp = load_input("inputfiles/day18/input.txt");
    //
    let h = inp.len();
    let w = inp[0].len();
    //
    let mut buf = vec![vec![false; w]; h];
    //
    for _ in 0..100 {
        next_lights(&inp, &mut buf);
        let temp = inp;
        inp = buf;
        buf = temp;
    }
    //
    let ans = inp
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|&&x| x)
        .count();
    //
    println!("lights: {}", ans);
}

fn part2() {
    let mut inp = load_input("inputfiles/day18/input.txt");
    //
    let h = inp.len();
    let w = inp[0].len();
    //
    let mut buf = vec![vec![false; w]; h];
    //
    for _ in 0..100 {
        next_lights(&inp, &mut buf);
        buf[0][0] = true;
        buf[h - 1][0] = true;
        buf[0][w - 1] = true;
        buf[h - 1][w - 1] = true;
        let temp = inp;
        inp = buf;
        buf = temp;
    }
    //
    let ans = inp
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|&&x| x)
        .count();
    //
    println!("lights: {}", ans);
}
