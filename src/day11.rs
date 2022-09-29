use std::{
    simd::{
        LaneCount, Mask, Simd, SimdPartialEq, SimdPartialOrd,
        SupportedLaneCount,
    },
    time::Instant,
    str,
};

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

fn increment<const N: usize>(pass: &mut [u8; N]) {
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

#[inline]
fn increment_vec<const N: usize, const M: usize>(
    pass_vec: &mut [Simd<u8, N>; M],
) where
    LaneCount<N>: SupportedLaneCount,
{
    pass_vec[0] += Simd::splat((N % 26) as u8);
    pass_vec[1] += Simd::splat((N / 26) as u8);

    let mut carry = Mask::splat(false);

    let s26 = Simd::splat(26);

    for v in pass_vec {
        let nv = *v + carry.select(Simd::splat(1), Simd::splat(0));
        carry = nv.simd_ge(s26);
        *v = nv % s26;
    }
}

fn isvalid<const N: usize>(pass: &[u8; N]) -> bool {
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

#[inline]
fn isvalid_vec<const N: usize, const M: usize>(
    pass_vec: &[Simd<u8, N>; M],
) -> Mask<i8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let r1 = {
        let mut counter = Simd::splat(0);
        let mut r1_buf = Mask::splat(false);

        for (a, b) in pass_vec.iter().zip(pass_vec.iter().skip(1)) {
            let rising_mask = (a - b).simd_eq(Simd::splat(1));
            counter = rising_mask.select(counter, Simd::splat(0));
            counter += rising_mask.select(Simd::splat(1i8), Simd::splat(0i8));
            r1_buf |= counter.simd_eq(Simd::splat(3));
        }

        r1_buf
    };

    let r2 = pass_vec
        .iter()
        .map(|s| {
            s.simd_eq(Simd::splat(8))
                | s.simd_eq(Simd::splat(11))
                | s.simd_eq(Simd::splat(14))
        })
        .fold(Mask::splat(false), |a, b| a | b);

    let r3 = {
        let mut last = Simd::splat(0);
        let mut b = Mask::splat(false);
        let mut amt = Simd::splat(0);

        for &v in pass_vec {
            let found = b & last.simd_eq(v);

            last = found.select(Simd::splat(0), v);
            b = !found;
            amt += found.select(Simd::splat(1u8), Simd::splat(0u8));
        }

        amt.simd_ge(Simd::splat(2))
    };

    r1 & (!r2) & r3
    // r1
}

fn part1_naive() -> String {
    (0..)
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
        .collect::<String>()
}

#[inline]
fn find_next_chunk_with_valid<const N: usize, const M: usize>(
    pass: &mut [Simd<u8, N>; M],
) where
    LaneCount<N>: SupportedLaneCount,
{
    increment_vec(pass);
    while !(isvalid_vec(pass).any()) {
        increment_vec(pass);
    }
}

fn solve_opt<const N_ITER: usize>() -> [u8; 8] {
    const N: usize = 32;

    let mut inp = get_input();
    let mut buf = [[0; N]; 8];

    for i in 0..N {
        increment(&mut inp);
        for (j, &c) in inp.iter().enumerate() {
            buf[j][i] = c;
        }
    }

    let mut pass = buf.map(|v| Simd::from(v));

    for _ in 0..N_ITER {
        find_next_chunk_with_valid(&mut pass);
    }

    let vmask = isvalid_vec(&pass);
    let ind = unsafe {
        vmask
            .to_array()
            .iter()
            .enumerate()
            .filter(|(_, &x)| x)
            .next()
            .unwrap()
            .0
    };

    let mut ans = [0; 8];

    for (v, x) in pass.iter().zip(&mut ans) {
        *x = v.as_array()[ind];
    }

    ans.reverse();

    ans.map(|x| x + b'a')
}

fn part1() {
    let t = Instant::now();
    let ans_naive = part1_naive();
    println!("Naive: {ans_naive} took {:?}", t.elapsed());

    let t = Instant::now();
    let ans_opt = solve_opt::<1>();
    let ans_opt = str::from_utf8(&ans_opt).unwrap();
    println!("Opt:   {ans_opt} took {:?}", t.elapsed());
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
