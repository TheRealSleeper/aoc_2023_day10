// Challenge found at https://adventofcode.com/2023/day/10
use std::env::args;
use std::fs::read_to_string;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
enum Pipe {
    Entry,
    Vertical,
    Horizontal,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
    Ground,
}

impl Pipe {
    fn from_char(section: char) -> Pipe {
        match section {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::RightUp,
            'J' => Pipe::LeftUp,
            '7' => Pipe::LeftDown,
            'F' => Pipe::RightDown,
            'S' => Pipe::Entry,
            '.' => Pipe::Ground,
            _ => panic!("Invalid charcter!"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn main() {
    let mut part1 = false;
    let mut _part2 = false;
    let mut path: Option<String> = None;
    let mut verbose = false;
    let mut args = args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--open" => path = Some(args.next().expect("No path provided!")),
            "-p1" | "--part1" => part1 = true,
            "-p2" | "--part2" => _part2 = true,
            "-v" | "--verbose" => verbose = true,
            "-h" | "--help" => {
                print!(
                    "This solves the challenge found at https://adventofcode.com/2023/day/10\n\
                    The following are valid commands:\n\
                    -o  | --open    : Uses specified input file\n\
                    -h  | --help    : opens this help page\n\
                    -p1 | --part1   : solves part 1 of the challenge
                    -p2 | --part2   : solves part 2 of the challenge (not implemented yet)\n\
                    -v  | --verbose : prints each pipe that gets checked\n"
                );
                return;
            }
            _ => {}
        }
    }

    let content = if let Some(p) = path {
        read_to_string(p).expect("Failed to read file")
    } else {
        println!("Using the more complex example");
        "..F7.\n\
         .FJ|.\n\
         SJ.L7\n\
         |F--J\n\
         LJ..."
            .to_string()
    };

    let mut map = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Pipe::from_char(c))
                .collect::<Vec<Pipe>>()
        })
        .collect::<Vec<Vec<Pipe>>>();

    let empty_row = vec![Pipe::Ground; map[0].len()];
    map.insert(0, empty_row.clone());
    map.push(empty_row.clone());

    // Add extra space around all pipes to ensure there will be no out of range indexes
    for row in &mut map {
        row.insert(0, Pipe::Ground);
        row.push(Pipe::Ground);
    }

    let mut starting_x = 0;
    let mut starting_y = 0;

    'search_row: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Pipe::Entry {
                starting_x = x;
                starting_y = y;
                break 'search_row;
            }
        }
    }

    if part1 {
        let mut steps: usize = 0;
        let mut prev_direction = Direction::None;
        let mut x = starting_x;
        let mut y = starting_y;
        'search: loop {
            if verbose {
                println!("Currently at row {}, column {}", y, x);
            }

            match prev_direction {
                Direction::None => {
                    if [Pipe::RightDown, Pipe::RightUp, Pipe::Horizontal]
                        .contains(&map[y][x - 1]) 
                    {
                        prev_direction = Direction::Left;
                        x -= 1;
                    } else if [Pipe::RightDown, Pipe::LeftDown, Pipe::Vertical]
                        .contains(&map[y - 1][x])
                    {
                        prev_direction = Direction::Up;
                        y -= 1;
                    } else if [Pipe::LeftDown, Pipe::LeftUp, Pipe::Horizontal]
                        .contains(&map[x + 1][x])
                    {
                        prev_direction = Direction::Right;
                        x += 1;
                    } else if [Pipe::RightUp, Pipe::LeftUp, Pipe::Vertical]
                        .contains(&map[y + 1][x])
                    {
                        prev_direction = Direction::Down;
                        y += 1;
                    } else {
                        panic!(
                            "No connection available at row {}, column {} after {} steps",
                            y, x, steps
                        );
                    }
                }

                Direction::Down => match map[y][x] {
                    Pipe::Vertical => {
                        prev_direction = Direction::Down;
                        y += 1;
                    }
                    Pipe::LeftUp => {
                        prev_direction = Direction::Left;
                        x -= 1;
                    }
                    Pipe::RightUp => {
                        prev_direction = Direction::Right;
                        x += 1;
                    }
                    _ => panic!(
                        "No connection available at row {}, column {} after {} steps",
                        y, x, steps
                    ),
                },

                Direction::Right => match map[y][x] {
                    Pipe::Horizontal => {
                        prev_direction = Direction::Right;
                        x += 1;
                    }
                    Pipe::LeftUp => {
                        prev_direction = Direction::Up;
                        y -= 1;
                    }
                    Pipe::LeftDown => {
                        prev_direction = Direction::Down;
                        y += 1;
                    }
                    _ => panic!(
                        "No connection available at row {}, column {} after {} steps",
                        y, x, steps
                    ),
                },

                Direction::Up => match map[y][x] {
                    Pipe::Vertical => {
                        prev_direction = Direction::Up;
                        y -= 1;
                    }
                    Pipe::LeftDown => {
                        prev_direction = Direction::Left;
                        x -= 1;
                    }
                    Pipe::RightDown => {
                        prev_direction = Direction::Right;
                        x += 1;
                    }
                    _ => panic!(
                        "No connection available at row {}, column {} after {} steps",
                        y, x, steps
                    ),
                },

                Direction::Left => match map[y][x] {
                    Pipe::Horizontal => {
                        prev_direction = Direction::Left;
                        x -= 1;
                    }
                    Pipe::RightUp => {
                        prev_direction = Direction::Up;
                        y -= 1;
                    }
                    Pipe::RightDown => {
                        prev_direction = Direction::Down;
                        y += 1;
                    }
                    _ => panic!(
                        "No connection available at row {}, column {} after {} steps",
                        y, x, steps
                    ),
                },
            }

            steps += 1;

            if map[y][x] == Pipe::Entry {
                // The furthest possible distance away from the start will be after half the steps to make a full loop
                println!("It took {} steps to get as far away as possible", steps / 2);
                break 'search;
            }
        }
    }
}
