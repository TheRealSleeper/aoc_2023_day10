// Challenge found at https://adventofcode.com/2023/day/10
use std::env::args;
use std::fs::read_to_string;

#[derive(Clone)]
#[allow(dead_code)]
struct Pipe {
    raw_char: char,
    conn_up: bool,
    conn_down: bool,
    conn_left: bool,
    conn_right: bool,
    entrance: bool,
    steps: usize,
    enter_direction: Direction
}

impl Pipe {
    fn from_char(inp: char) -> Pipe {
        match inp {
            '|' => Pipe {
                raw_char: inp,
                conn_up: true,
                conn_down: true,
                conn_left: false,
                conn_right: false,
                entrance: false,
                steps: 0,
                enter_direction: Direction::None
            },
            '-' => Pipe {
                raw_char: inp,
                conn_up: false,
                conn_down: false,
                conn_left: true,
                conn_right: true,
                entrance: false,
                steps: 0,
                enter_direction: Direction::None
            },
            'L' => Pipe {
                raw_char: inp,
                conn_up: true,
                conn_down: false,
                conn_left: false,
                conn_right: true,
                entrance: false,
                steps: 0,
                enter_direction: Direction::None
            },
            'J' => Pipe {
                raw_char: inp,
                conn_up: true,
                conn_down: false,
                conn_left: true,
                conn_right: false,
                entrance: false,
                steps: 0,
                enter_direction: Direction::None
            },
            '7' => Pipe {
                raw_char: inp,
                conn_up: false,
                conn_down: true,
                conn_left: true,
                conn_right: false,
                entrance: false,
                steps: 0,
                enter_direction: Direction::None
            },
            'F' => Pipe {
                raw_char: inp,
                conn_up: false,
                conn_down: true,
                conn_left: false,
                conn_right: true,
                entrance: false,
                steps: 0,
                enter_direction: Direction::None
            },
            'S' => Pipe {
                raw_char: inp,
                conn_up: true,
                conn_down: true,
                conn_left: true,
                conn_right: true,
                entrance: true,
                steps: 0,
                enter_direction: Direction::None
            },
            _ => Pipe {
                raw_char: inp,
                conn_up: false,
                conn_down: false,
                conn_left: false,
                conn_right: false,
                entrance: false,
                steps: 0,
                enter_direction: Direction::None
            },
        }
    }

    fn new() -> Pipe {
        Pipe {
            raw_char: '.',
            conn_up: false,
            conn_down: false,
            conn_left: false,
            conn_right: false,
            entrance: false,
            steps: 0,
            enter_direction: Direction::None
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}



fn main() {
    let mut part1 = true;
    let mut path: Option<String> = None;
    let mut verbose = false;

    let env_args = args().collect::<Vec<String>>();

    for i in 0..env_args.len() {
        match env_args[i].as_str() {
            "-o" | "--open" => path = Some(env_args[i + 1].clone()),
            "--part2" => part1 = false,
            "-v" | "--verbose" => verbose = true,
            "-h" | "--help" => {
                print!(
                    "This solves the challenge found at https://adventofcode.com/2023/day/10\n\
                The following are valid commands:\n\
                -o or --open (input file path): Uses specified input file; \
                if not specified the sample input will be used\n\
                -h or --help: opens this help page\n\
                --part 2: solves part 2 of the challenge (not implemented yet)\n\
                -v or --verbose: prints each pipe that gets checked\n"
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
        let sample = "..F7.\n\
            .FJ|.\n\
            SJ.L7\n\
            |F--J\n\
            LJ..."
            .to_string();
        sample.clone()
    };

    let mut map = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Pipe::from_char(c))
                .collect::<Vec<Pipe>>()
        })
        .collect::<Vec<Vec<Pipe>>>();

    let empty_row = vec![Pipe::new(); map[0].len()];
    map.insert(0, empty_row.clone());
    map.push(empty_row.clone());

    // Add extra space around all pipes to ensure there will be no out of range indexes
    for row in &mut map {
        row.insert(0, Pipe::new());
        row.push(Pipe::new());
    }

    if part1 {
        let mut starting_x= 0;
        let mut starting_y= 0;

        'search_row: for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x].entrance {
                    starting_x = x;
                    starting_y = y;
                    break 'search_row;
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
            if prev_direction == Direction::Right {
                if map[y][x].conn_up && map[y - 1][x].conn_down {
                    prev_direction = Direction::Up;
                    map[y][x].steps = steps;
                    y -= 1;
                } else if map[y][x].conn_down && map[y + 1][x].conn_up {
                    prev_direction = Direction::Down;
                    map[y][x].steps = steps;
                    y += 1;
                } else if map[y][x].conn_right && map[y][x + 1].conn_left {
                    prev_direction = Direction::Right;
                    map[y][x].steps = steps;
                    x += 1;
                } else {
                    panic!("No connection available at row {}, column {} after {} steps", y, x, steps);
                }
            } else if prev_direction == Direction::Left {
                if map[y][x].conn_up && map[y - 1][x].conn_down {
                    prev_direction = Direction::Up;
                    map[y][x].steps = steps;
                    y -= 1;
                } else if map[y][x].conn_down && map[y + 1][x].conn_up {
                    prev_direction = Direction::Down;
                    map[y][x].steps = steps;
                    y += 1;
                } else if map[y][x].conn_left && map[y][x - 1].conn_right {
                    prev_direction = Direction::Left;
                    map[y][x].steps = steps;
                    x -= 1;
                } else {
                    panic!("No connection available at row {}, column {} after {} steps", y, x, steps);
                }
            } else if prev_direction == Direction::Up {
                if map[y][x].conn_up && map[y - 1][x].conn_down {
                    prev_direction = Direction::Up;
                    map[y][x].steps = steps;
                    y -= 1;
                } else if map[y][x].conn_left && map[y][x - 1].conn_right {
                    prev_direction = Direction::Left;
                    map[y][x].steps = steps;
                    x -= 1;
                } else if map[y][x].conn_right && map[y][x + 1].conn_left {
                    prev_direction = Direction::Right;
                    map[y][x].steps = steps;
                    x += 1;
                } else {
                    panic!("No connection available at row {}, column {} after {} steps", y, x, steps);
                }
            } else if prev_direction == Direction::Down {
                if map[y][x].conn_left && map[y][x - 1].conn_right {
                    prev_direction = Direction::Left;
                    map[y][x].steps = steps;
                    x -= 1;
                } else if map[y][x].conn_down && map[y + 1][x].conn_up {
                    prev_direction = Direction::Down;
                    map[y][x].steps = steps;
                    y += 1;
                } else if map[y][x].conn_right && map[y][x + 1].conn_left {
                    prev_direction = Direction::Right;
                    map[y][x].steps = steps;
                    x += 1;
                } else {
                    panic!("No connection available at row {}, column {} after {} steps", y, x, steps);
                }
            } else if prev_direction == Direction::None {
                if map[y][x].conn_left && map[y][x - 1].conn_right {
                    prev_direction = Direction::Left;
                    map[y][x].steps = steps;
                    x -= 1;
                } else if map[y][x].conn_down && map[y + 1][x].conn_up {
                    prev_direction = Direction::Down;
                    map[y][x].steps = steps;
                    y += 1;
                } else if map[y][x].conn_right && map[y][x + 1].conn_left {
                    prev_direction = Direction::Right;
                    map[y][x].steps = steps;
                    x += 1;
                } else {
                    panic!("No connection available at row {}, column {} after {} steps", y, x, steps);
                }
                steps += 1;
                continue 'search;
            }

            steps += 1;
            if map[y][x].entrance {
                // The furthest possible distance away from the start will be after half the steps to make a full loop
                println!("It took {} steps to get as far away as possible", steps/2);
                break 'search;
            }
        }
    }
}
