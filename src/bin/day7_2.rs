use std::iter::zip;
use std::iter::Iterator;


fn solve(t: u64, n: Vec<u64>) -> bool {
    if n.len() >= 2 {
        let a = n[0];
        let b = n[1];
        let mut va = n[2..].to_vec();
        let mut vm = n[2..].to_vec();
        let mut vc = n[2..].to_vec();
        va.insert(0, a + b);
        vm.insert(0, a * b);
        vc.insert(0, a * 10u64.pow(((b as f64).log10().floor() + 1f64) as u32) + b);
        if solve(t, va) {
            true
        } else if solve(t, vm) {
            true
        } else if solve(t, vc) {
            true
        } else {
            false
        }
    } else {
        t == n[0]
    }
}


fn main() {
    let mut tests = Vec::new();
    let mut numbers: Vec<_> = Vec::new();

    let _: Vec<_> = include_str!("../../data/day7.txt")
        .lines()
        .map(|line| {
            let (test_str, numbers_str) = line.split_once(':').unwrap();
            tests.push(test_str.parse::<u64>().unwrap());
            numbers.push(numbers_str
                .trim_start()
                .split(' ')
                .map(|ns| ns.parse::<u64>().unwrap()).collect::<Vec<_>>());
        }).collect();

    println!("{}",
        zip(tests, numbers)
        .map(|(test, numbers)|
            if solve(test, numbers) { test } else { 0 }
        )
        .sum::<u64>()
    );
}
