use itertools::Itertools;

fn main() {
    let (rules_str, updates_str) = include_str!("../day5.txt").split_once("\n\n").unwrap();

    let rules: Vec<_> = rules_str
        .lines()
        .map(|line| { line.split('|').map(|ns| ns.parse::<i32>().unwrap()).collect::<Vec<_>>() })
        .collect();

    let updates: Vec<_> = updates_str
        .lines()
        .map(|line| { line.split(',').map(|ns| ns.parse::<i32>().unwrap()).collect::<Vec<_>>() })
        .collect();

    println!("sum: {}", updates
        .into_iter()
        .map(|upd| {
            if upd
                .clone().into_iter().combinations(2)
                .any(|cmb| {
                    rules.contains(&cmb.into_iter().rev().collect::<Vec<_>>())
                }) {
                    0
                } else {
                    upd[upd.len() / 2]
                }
        })
        .sum::<i32>()
    );
}
