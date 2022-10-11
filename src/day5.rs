use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader},
    simd::{
        LaneCount, Mask, Simd, SimdPartialEq, SimdPartialOrd,
        SupportedLaneCount,
    },
    time::Instant,
};

use itertools::Itertools;

use lazy_static::*;

use crate::transpose_u8::transpose_64x16_u8;

pub const PARTS: [fn(); 2] = [part1, part2];

fn isnice_naive(s: &str) -> bool {
    lazy_static! {
        static ref VOWELS: HashSet<char> = "aeiou".chars().collect();
        static ref BLACKLIST: HashSet<(char, char)> =
            [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]
                .iter()
                .cloned()
                .collect();
    }
    let r1 = s.chars().filter(|c| VOWELS.contains(&c)).count() >= 3;
    let r2 = s.chars().tuple_windows().any(|(a, b)| a == b);
    let r3 = s.chars().tuple_windows().all(|s| !BLACKLIST.contains(&s));
    r1 && r2 && r3
}

fn isnice_opt(s: &str) -> bool {
    let r1 = s
        .chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        >= 3;
    let r2 = s.chars().tuple_windows().any(|(a, b)| a == b);
    let r3 = s.chars().tuple_windows().all(|s| {
        !matches!(s, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y'))
    });
    r1 && r2 && r3
}

fn isnice_byte(s: &[u8]) -> bool {
    let r1 = s
        .iter()
        .filter(|c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
        .count()
        >= 3;
    let r2 = s.iter().tuple_windows().any(|(a, b)| a == b);
    let r3 = s.iter().tuple_windows().all(|s| {
        !matches!(s, (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y'))
    });
    r1 && r2 && r3
}

fn isnice_simd<const LANES: usize, const N: usize>(
    s: &[Simd<u8, LANES>; N],
) -> Mask<i8, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let r1 = {
        let mut counter = Simd::splat(0u8);

        for v in s {
            let isvowel = v.simd_eq(Simd::splat(b'a'))
                | v.simd_eq(Simd::splat(b'e'))
                | v.simd_eq(Simd::splat(b'i'))
                | v.simd_eq(Simd::splat(b'o'))
                | v.simd_eq(Simd::splat(b'u'));

            counter += isvowel.select(Simd::splat(1), Simd::splat(0));
        }

        counter.simd_ge(Simd::splat(3))
    };

    let r2 = {
        let mut acc = Mask::splat(false);

        for i in 0..N - 1 {
            acc |= s[i].simd_eq(s[i + 1]);
        }

        acc
    };

    let r3 = {
        let blacklisted = |a: Simd<u8, LANES>, b: Simd<u8, LANES>| {
            a.simd_eq(Simd::splat(b'a')) & b.simd_eq(Simd::splat(b'b'))
                | a.simd_eq(Simd::splat(b'c')) & b.simd_eq(Simd::splat(b'd'))
                | a.simd_eq(Simd::splat(b'p')) & b.simd_eq(Simd::splat(b'q'))
                | a.simd_eq(Simd::splat(b'x')) & b.simd_eq(Simd::splat(b'y'))
        };

        let mut acc = Mask::splat(true);

        for i in 0..N - 1 {
            acc &= !blacklisted(s[i], s[i + 1]);
        }

        acc
    };

    r1 & r2 & r3
}

fn part1_simd(inp: &str) -> usize {
    const N: usize = 64;

    let inp = inp.as_bytes();
    let (inp_cs, inp_r) =
        unsafe { inp.as_chunks_unchecked::<17>().as_chunks::<N>() };

    let mut count = Simd::<u8, N>::splat(0);

    let mut inp_v = [[0; N]; 16];

    for inp_c in inp_cs {
        let inp_ca: [[u8; 16]; N] = inp_c.map(|v| v.as_chunks().0[0]);

        transpose_64x16_u8::trans(&inp_ca, &mut inp_v);

        let inp_s = inp_v.map(|v| Simd::from(v));

        let nice = isnice_simd(&inp_s);

        count += nice.select(Simd::splat(1), Simd::splat(0));
    }

    count.to_array().iter().map(|&x| x as usize).sum::<usize>()
        + inp_r.iter().filter(|&s| isnice_byte(&s[0..16])).count()
}

fn part1() {
    let inp = fs::read_to_string("inputfiles/day5/input.txt").unwrap();

    let t = Instant::now();
    let ans = inp.split('\n').filter(|l| isnice_naive(&l)).count();
    let t = t.elapsed();
    println!("Naive: {ans} took {t:?}");

    let t = Instant::now();
    let ans = inp.split('\n').filter(|l| isnice_opt(&l)).count();
    let t = t.elapsed();
    println!("  Opt: {ans} took {t:?}");

    let t = Instant::now();
    let ans = part1_simd(&inp);
    let t = t.elapsed();
    println!("  Opt: {ans} took {t:?}");
}

fn isreallynice(s: &str) -> bool {
    let r1 = s
        .chars()
        .tuple_windows()
        .enumerate()
        .scan(HashMap::new(), |buf, (i, (a, b))| {
            if let Some(v) = buf.get_mut(&(a, b)) {
                Some((i as isize - *v as isize).abs() >= 2)
            } else {
                buf.insert((a, b), i);
                Some(false)
            }
        })
        .any(|b| b);
    let r2 = s.chars().tuple_windows().any(|(a, _, b)| a == b);
    r1 && r2
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day5/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| isreallynice(&l))
        .count();
    println!("{}", ans);
}
