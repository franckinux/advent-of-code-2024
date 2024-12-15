use itertools::Itertools;

fn main() {
    println!("{}",
        include_str!("../../data/day2.txt")
            .lines()
            .fold(0, |acc, line| {
                let items: Vec<i64> = line.split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();

                for r in 0..items.len() {
                    let items_: Vec<i64> = items.clone().into_iter().enumerate().filter(|&(i, _)| i != r).map(|(_, v)| v).collect();

                    let diffs: Vec<_> = items_.into_iter().tuple_windows::<(_, _)>().map(|(d1, d2)| d2 - d1).collect();
                    if diffs.iter().all(|d| (1..=3).contains(&d.abs())) && (diffs.iter().all(|d| *d > 0) || diffs.iter().all(|d| *d < 0)) {
                        return acc + 1
                    }
                };
                acc
            })
    );
}
