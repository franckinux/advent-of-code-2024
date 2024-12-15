fn main() {
    println!("sum of mul: {}",
        include_str!("../../data/day3.txt")
        .lines()
        .fold(0, |acc, line| {
            let mut n1 = 0u32;
            let mut n2 = 0u32;
            let mut state = ' ';
            acc + line.chars().fold(0, |acc, c| {
                if c == 'm' {state = 'm'; acc }
                else if state == 'm' && c == 'u' { state = 'u'; acc }
                else if state == 'u' && c == 'l' { state = 'l'; acc }
                else if state == 'l' && c == '(' { n1 = 0; state = '('; acc }
                else if c.is_digit(10) {
                    if state == '(' || state == 'c' {
                        state = 'c';
                        n1 = n1 * 10 + c.to_digit(10).unwrap();
                        acc
                    } else if state == ',' || state == 'd' {
                        state = 'd';
                        n2 = n2 * 10 + c.to_digit(10).unwrap();
                        acc
                    } else {
                        acc
                    }
                }
                else if state == 'c' && c == ',' { n2 = 0; state = ','; acc }
                else if state == 'd' && c == ')' { state = ' ' ; acc + n1 * n2 }
                else { state = ' '; acc }
            })
        })
    );
}
