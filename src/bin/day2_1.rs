fn main() {
    println!("number of safe reports: {}",
        include_str!("../../data/day2.txt")
            .lines()
            .fold(0, |acc, line| {
                let items: Vec<i64> = line.split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();

                for r in 0..items.len() {
                    let mut diffs = Vec::new();
                    for i in 0..items.len() - 1 {
                        diffs.push(items[i+1] - items[i]);
                    }
                    if diffs.iter().all(|d| (1..=3).contains(&d.abs())) && (diffs.iter().all(|d| *d > 0) || diffs.iter().all(|d| *d < 0)) {
                        return acc + 1
                    }
                };
                acc
            })
    );
}
