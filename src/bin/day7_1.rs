use std::iter::zip;
use std::iter::Iterator;


struct BinaryIterator {
    number: u64,
    iterations: usize,
}


impl Iterator for BinaryIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations == 0 {
            None
        } else {
            self.iterations -= 1;
            let bit = self.number & 1 != 0;
            self.number >>= 1;
            Some(bit)
        }
    }
}

impl BinaryIterator {
    fn new(number: u64, iterations: usize) -> Self {
        Self{number, iterations}
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

    let mut result = 0;
    for (test, numbers) in zip(tests, numbers) {
        let size = numbers.len();
        for n in 0..2u64.pow(size as u32) {
            let mut t = 0u64;
            for (i, b) in BinaryIterator::new(n, size).enumerate() {
                if b { t += numbers[i] } else { t *= numbers[i] }
            };
            if t == test {
                result += test;
                break;
            }
        };
    };
    println!("{}", result);
}
