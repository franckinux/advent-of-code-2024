pub fn main() {
    let mut map: Vec<_> = include_str!("../../data/day6.txt")
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();
    let height = map.len();
    let width = map[0].len();

    // find initial position of the guard
    let mut pos = map
        .iter()
        .enumerate()
        .filter_map(|(index, row)| if let Some(pos) = row.iter().position(|c| *c == '^') {Some((index, pos))} else {None})
        .collect::<Vec<_>>()[0];

    let mut m = '^';
    let mut mov = (0i16, 0i16);
    loop {
        map[pos.0][pos.1] = 'X';
        match m {
            '^' => { if pos.0 == 0 { break; } else { mov = (-1, 0); }},
            'v' => { if pos.0 == height - 1 { break; } else {mov = (1, 0); }},
            '<' => { if pos.1 == 0 { break; } else { mov = (0, -1); }},
            '>' => { if pos.1 == width - 1 { break; }  else { mov = (0, 1); }},
            _ => { mov = (0, 0); }
        };
        if map[(pos.0 as i16 + mov.0) as usize][(pos.1 as i16 + mov.1) as usize] == '#' {
            match m {
                '^' => m = '>',
                '>' => m = 'v',
                'v' => m = '<',
                '<' => m = '^',
                _ => m = ' ',
            };
        } else {
            pos.0 = (pos.0 as i16 + mov.0) as usize;
            pos.1 = (pos.1 as i16 + mov.1) as usize;
        };
    };
    // println!("{:?}", map);

    println!("number of visited positions: {}",
        map.into_iter()
        .fold(0, |acc, row| {
            acc + row.iter().filter(|c| **c == 'X').count()
        })
    );
}
