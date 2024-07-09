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
    let mut part2 = false;
    let mut path: Option<String> = None;
    let mut verbose = false;
    let mut args = args().skip(1);

    let help_txt = "This solves the challenge found at https://adventofcode.com/2023/day/10\n\
        The following are valid commands:\n\
        -o  | --open    : Uses specified input file\n\
        -h  | --help    : opens this help page\n\
        -1 | --part1   : solves part 1 of the challenge
        -2 | --part2   : solves part 2 of the challenge (not implemented yet)\n\
        -v  | --verbose : prints each pipe that gets checked\n\n\
        Choose only part 1 or part 2 if using sample input, as the sample are different.\n";

    while let Some(arg) = args.next() {
        if arg.chars().nth(0).unwrap() == '-' {
            if arg.chars().nth(1).unwrap_or('\0') != '-' {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'i' => path = Some(args.next().expect("No path provided!")),
                        '1' => part1 = true,
                        '2' => part2 = true,
                        'v' => verbose = true,
                        'h' => {
                            println!("{}", help_txt);
                            return;
                        }
                        _ => println!("Ignoring unkown option '{}'", ch),
                    }
                }
            } else {
                let st = arg.chars().skip(2).collect::<String>();
                match st.as_str() {
                    "input" => path = Some(args.next().expect("No path provided!")),
                    "part1" => part1 = true,
                    "part2" => part2 = true,
                    "verbose" => verbose = true,
                    "help" => {
                        print!(
                            "This solves the challenge found at https://adventofcode.com/2023/day/10\n\
                            The following are valid options:\n\
                            -i  | --input    : Uses specified input file\n\
                            -h  | --help    : opens this help page\n\
                            -1 | --part1   : solves part 1 of the challenge
                            -2 | --part2   : solves part 2 of the challenge (not implemented yet)\n\
                            -v  | --verbose : prints each pipe that gets checked\n\n\
                            Choose only part 1 or part 2 if using sample input, as the sample inputs are different.\n"
                        );
                        return;
                    }
                    _ => println!("Ignoring unkown option '{}'", st),
                }
            }
        }
    }

    let content = if let Some(p) = path {
        read_to_string(p).expect("Failed to read file")
    } else if part1 {
        // Takes 8 steps
        "..F7.\n\
         .FJ|.\n\
         SJ.L7\n\
         |F--J\n\
         LJ..."
            .to_string()
    } else {
        // Encloses 10 tiles
        "FF7FSF7F7F7F7F7F---7\n\
         L|LJ||||||||||||F--J\n\
         FL-7LJLJ||||||LJL-77\n\
         F--JF--7||LJLJ7F7FJ-\n\
         L---JF-JLJ.||-FJLJJ7\n\
         |F|F-JF---7F7-L7L|7|\n\
         |FFJF7L7F-JF7|JL---7\n\
         7-L-JL7||F7|L7F-7F7|\n\
         L.L7LFJ|||||FJL7||LJ\n\
         L7JLJL-JLJLJL--JLJ.L"
            .to_string()
    };

    let mut map = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (Pipe::from_char(c), false))
                .collect::<Vec<(Pipe, bool)>>()
        })
        .collect::<Vec<Vec<(Pipe, bool)>>>();

    let empty_row = vec![(Pipe::Ground, false); map[0].len()];
    map.insert(0, empty_row.clone());
    map.push(empty_row.clone());

    // Add extra space around all pipes to ensure there will be no out of range indexes
    for row in &mut map {
        row.insert(0, (Pipe::Ground, false));
        row.push((Pipe::Ground, false));
    }

    let mut starting_x = 0;
    let mut starting_y = 0;

    'search_start: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x].0 == Pipe::Entry {
                map[y][x].1 = true;
                starting_x = x;
                starting_y = y;
                break 'search_start;
            }
        }
    }

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
                if [Pipe::RightDown, Pipe::RightUp, Pipe::Horizontal].contains(&map[y][x - 1].0) {
                    prev_direction = Direction::Left;
                    x -= 1;
                } else if [Pipe::RightDown, Pipe::LeftDown, Pipe::Vertical]
                    .contains(&map[y - 1][x].0)
                {
                    prev_direction = Direction::Up;
                    y -= 1;
                } else if [Pipe::LeftDown, Pipe::LeftUp, Pipe::Horizontal]
                    .contains(&map[y][x + 1].0)
                {
                    prev_direction = Direction::Right;
                    x += 1;
                } else if [Pipe::RightUp, Pipe::LeftUp, Pipe::Vertical].contains(&map[y + 1][x].0) {
                    prev_direction = Direction::Down;
                    y += 1;
                } else {
                    panic!(
                        "No connection available at row {}, column {} after {} steps",
                        y, x, steps
                    );
                }
            }

            Direction::Down => match map[y][x].0 {
                Pipe::Vertical => {
                    prev_direction = Direction::Down;
                    map[y][x].1 = true;
                    y += 1;
                }
                Pipe::LeftUp => {
                    prev_direction = Direction::Left;
                    map[y][x].1 = true;
                    x -= 1;
                }
                Pipe::RightUp => {
                    prev_direction = Direction::Right;
                    map[y][x].1 = true;
                    x += 1;
                }
                _ => panic!(
                    "No connection available at row {}, column {} after {} steps",
                    y, x, steps
                ),
            },

            Direction::Right => match map[y][x].0 {
                Pipe::Horizontal => {
                    prev_direction = Direction::Right;
                    map[y][x].1 = true;
                    x += 1;
                }
                Pipe::LeftUp => {
                    prev_direction = Direction::Up;
                    map[y][x].1 = true;
                    y -= 1;
                }
                Pipe::LeftDown => {
                    prev_direction = Direction::Down;
                    map[y][x].1 = true;
                    y += 1;
                }
                _ => panic!(
                    "No connection available at row {}, column {} after {} steps",
                    y, x, steps
                ),
            },

            Direction::Up => match map[y][x].0 {
                Pipe::Vertical => {
                    prev_direction = Direction::Up;
                    map[y][x].1 = true;
                    y -= 1;
                }
                Pipe::LeftDown => {
                    prev_direction = Direction::Left;
                    map[y][x].1 = true;
                    x -= 1;
                }
                Pipe::RightDown => {
                    prev_direction = Direction::Right;
                    map[y][x].1 = true;
                    x += 1;
                }
                _ => panic!(
                    "No connection available at row {}, column {} after {} steps",
                    y, x, steps
                ),
            },

            Direction::Left => match map[y][x].0 {
                Pipe::Horizontal => {
                    prev_direction = Direction::Left;
                    map[y][x].1 = true;
                    x -= 1;
                }
                Pipe::RightUp => {
                    prev_direction = Direction::Up;
                    map[y][x].1 = true;
                    y -= 1;
                }
                Pipe::RightDown => {
                    prev_direction = Direction::Down;
                    map[y][x].1 = true;
                    y += 1;
                }
                _ => panic!(
                    "No connection available at row {}, column {} after {} steps",
                    y, x, steps
                ),
            },
        }

        steps += 1;

        if map[y][x].0 == Pipe::Entry {
            // The furthest possible distance away from the start will be after half the steps to make a full loop
            if part1 {
                println!("It took {} steps to get as far away as possible", steps / 2);
            }
            break 'search;
        }
    }

    if part2 {
        let mut interior_tiles = 0;

        // Replace the entry point with a directional Pipe so that counting works properly
        let mut left = false;
        let mut up = false;
        let mut right = false;
        let mut down = false;
        if [Pipe::RightDown, Pipe::RightUp, Pipe::Horizontal]
            .contains(&map[starting_y][starting_x - 1].0)
        {
            left = true;
        }

        if [Pipe::RightDown, Pipe::LeftDown, Pipe::Vertical].contains(&map[y - 1][x].0) {
            up = true;
        }

        if [Pipe::LeftDown, Pipe::LeftUp, Pipe::Horizontal].contains(&map[y][x + 1].0) {
            right = true;
        }

        if [Pipe::RightUp, Pipe::LeftUp, Pipe::Vertical].contains(&map[y + 1][x].0) {
            down = true;
        }

        if left && up {
            map[starting_y][starting_x].0 = Pipe::LeftUp;
        }
        if left && right {
            map[starting_y][starting_x].0 = Pipe::Horizontal;
        }
        if left && down {
            map[starting_y][starting_x].0 = Pipe::LeftDown;
        }
        if right && up {
            map[starting_y][starting_x].0 = Pipe::RightUp;
        }
        if right && down {
            map[starting_y][starting_x].0 = Pipe::RightDown;
        }
        if down && up {
            map[starting_y][starting_x].0 = Pipe::Vertical;
        }

        for y in 0..map.len() {
            let mut prev_turn: Option<Pipe> = None;
            let mut pipe_count = 0;

            for x in 0..map[y].len() {
                let current_tile = map[y][x].0.clone();
                let is_loop = map[y][x].1;

                if !is_loop && pipe_count % 2 == 1 {
                    interior_tiles += 1;
                }

                if is_loop {
                    if current_tile != Pipe::Horizontal {
                        pipe_count += 1;
                    }

                    if (prev_turn == Some(Pipe::RightUp) && current_tile == Pipe::LeftDown)
                        || (prev_turn == Some(Pipe::RightDown) && current_tile == Pipe::LeftUp)
                    {
                        pipe_count -= 1;
                    }

                    if current_tile == Pipe::RightUp {
                        prev_turn = Some(Pipe::RightUp);
                    } else if current_tile == Pipe::RightDown {
                        prev_turn = Some(Pipe::RightDown);
                    }

                    if verbose {
                        if (x, y) == (starting_x, starting_y) {
                            print!("S");
                        } else {
                            match current_tile {
                                Pipe::Entry => print!("S"),
                                Pipe::Ground => print!("O"),
                                Pipe::Horizontal => print!("═"),
                                Pipe::LeftDown => print!("╗"),
                                Pipe::LeftUp => print!("╝"),
                                Pipe::RightDown => print!("╔"),
                                Pipe::RightUp => print!("╚"),
                                Pipe::Vertical => print!("║"),
                            }
                        }
                    }
                } else if pipe_count % 2 == 1 && verbose {
                    print!("X");
                } else if verbose {
                    print!(".");
                }

                if current_tile == Pipe::Entry {
                    let _foo = "bar";
                }
            }

            if verbose {
                print!("\n");
            }
        }

        println!("There are {} interior tiles", interior_tiles);
    }
}
