use std::iter::zip;
use itertools::Itertools;


fn main() {
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    let _: Vec<_> = include_str!("../../data/day1.txt")
        .lines()
        .map(|line| {
            let (i1, i2) = line.split_whitespace().map(|x| x.parse().unwrap()).collect_tuple().unwrap();
            list1.push(i1);
            list2.push(i2);
        }).collect();
    list1.sort();
    list2.sort();

    println!("similarity score: {}", zip(list1, list2).map(|t| (t.0 - t.1).abs()).sum::<i64>());
}
