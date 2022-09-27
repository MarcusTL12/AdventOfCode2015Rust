pub const PARTS: [fn(); 2] = [part1, part2];

const INPUT: &[u8; 8] = b"hepxcrrq";

fn get_input() -> [u8; 8] {
    let mut inp = *INPUT;
    inp.reverse();

    for x in inp.iter_mut() {
        *x -= b'a';
    }

    inp
}

fn blacklist(c: u8) -> bool {
    matches!(c, 8 | 11 | 14)
}

fn increment(pass: &mut [u8]) {
    pass.iter_mut().fold(true, |mut s, x| {
        if s {
            *x += 1;
            *x %= 26;
            if *x != 0 {
                s = false;
            }
        }
        s
    });
}

fn isvalid(pass: &[u8]) -> bool {
    let (_, _, r1) =
        pass.iter().fold((0, 0, false), |(last, len, done), &x| {
            if x + 1 == last {
                (x, len + 1, (len) >= 2 || done)
            } else {
                (x, 1, done)
            }
        });
    //
    let r2 = pass.iter().any(|&x| blacklist(x));
    //
    let r3 = pass
        .iter()
        .fold((0, false, 0), |(last, b, amt), &x| {
            if b && last == x {
                (0, false, amt + 1)
            } else {
                (x, true, amt)
            }
        })
        .2
        >= 2;
    //
    r1 && !r2 && r3
}

fn part1() {
    let ans = (0..)
        .scan(get_input(), |buf, _| {
            increment(buf);
            Some(*buf)
        })
        .filter(|x| isvalid(x))
        .next()
        .unwrap()
        .iter()
        .rev()
        .map(|c| (c + b'a') as char)
        .collect::<String>();
    //
    println!("{}", ans);
}

fn part2() {
    let ans = (0..)
        .scan(get_input(), |buf, _| {
            increment(buf);
            Some(*buf)
        })
        .filter(|x| isvalid(x))
        .skip(1)
        .next()
        .unwrap()
        .iter()
        .rev()
        .map(|c| (c + b'a') as char)
        .collect::<String>();
    //
    println!("{}", ans);
}
