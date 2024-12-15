fn main() {
    let mut enable = true;
    println!("sum of mul: {}",
        include_str!("../day3.txt")
        .lines()
        .fold(0, |acc, line| {
            let mut n1 = 0u32;
            let mut n2 = 0u32;
            let mut state = ' ';
            acc + line.chars().fold(0, |acc, c| {
                if enable {
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
                    else if c == 'd' { state = 'd'; acc}
                    else if state == 'd' && c == 'o' { state = 'o'; acc}
                    else if state == 'o' && c == 'n' { state = 'n'; acc}
                    else if state == 'n' && c == '\'' { state = '\''; acc}
                    else if state == '\'' && c == 't' { state = 't'; acc}
                    else if state == 't' && c == '(' { state = '{'; acc}
                    else if state == '{' && c == ')' { state = ' '; enable = false; acc}
                    else { state = ' '; acc }
                } else {
                    if c == 'd' { state = 'd'; acc}
                    else if state == 'd' && c == 'o' { state = 'o'; acc}
                    else if state == 'o' && c == '(' { state = '['; acc}
                    else if state == '[' && c == ')' { state = ' '; enable = true; acc}
                    else { state = ' '; acc }
                }
            })
        })
    );
}
