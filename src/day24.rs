use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn combs<'a>(
    nums: &'a [u64],
    sum: u64,
    maxlen: usize,
    len: usize,
    off: usize,
) -> Box<dyn Iterator<Item = Vec<usize>> + 'a> {
    if sum == 0 {
        Box::from(iter::once(vec![]))
    } else if len >= maxlen {
        Box::from(iter::empty())
    } else {
        Box::from((0..nums.len()).filter(move |&i| nums[i] <= sum).flat_map(
            move |i| {
                combs(
                    &nums[i + 1..],
                    sum - nums[i],
                    maxlen,
                    len + 1,
                    off + i + 1,
                )
                .map(move |mut v| {
                    v.push(i + off);
                    v
                })
            },
        ))
    }
}

fn exist_of_len(nums: &[u64], sum: u64, len: usize) -> bool {
    match (sum, len) {
        (0, 0) => return true,
        (_, 0) => return false,
        _ => (),
    }
    //
    if let Some(i) = (0..nums.len() - len + 1)
        .skip_while(|&i| nums[i] > sum)
        .next()
    {
        for i in i..nums.len() - len + 1 {
            if exist_of_len(&nums[i + 1..], sum - nums[i], len - 1) {
                return true;
            }
        }
        false
    } else {
        false
    }
}

fn part1() {
    let numbers: Vec<u64> =
        BufReader::new(File::open("inputfiles/day24/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.parse().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
    //
    let sum: u64 = numbers.iter().sum();
    //
    let amt = (0..)
        .skip_while(|&i| !exist_of_len(&numbers, sum / 3, i))
        .next()
        .unwrap();
    //
    let ans = combs(&numbers, sum / 3, amt, 0, 0)
        .map(|v| v.iter().map(|&i| numbers[i]).product::<u64>())
        .min()
        .unwrap();
    //
    println!("{}", ans);
}

fn part2() {
    let numbers: Vec<u64> =
        BufReader::new(File::open("inputfiles/day24/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.parse().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
    //
    let sum: u64 = numbers.iter().sum();
    //
    let amt = (0..)
        .skip_while(|&i| !exist_of_len(&numbers, sum / 4, i))
        .next()
        .unwrap();
    //
    let ans = combs(&numbers, sum / 4, amt, 0, 0)
        .map(|v| v.iter().map(|&i| numbers[i]).product::<u64>())
        .min()
        .unwrap();
    //
    println!("{}", ans);
}
