use library::{get_filename_arg, get_two_dimensional_vector};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    let mut input = get_two_dimensional_vector(&file_name);

    let start_location = find_start(&input);

    let (mut current_row, mut current_column, mut current_direction) =
        get_starting_direction(start_location, &input);

    let starting_direction = current_direction.clone();

    let mut count = 1;
    while input[current_row][current_column] != 'S' {
        count += 1;
        (current_row, current_column, current_direction) =
            get_next_location(current_row, current_column, current_direction, &mut input);
    }

    println!("Part 1: {}", count / 2);

    match current_direction {
        Directions::North => match starting_direction {
            Directions::North => {
                input[current_row][current_column] = '!';
            }
            Directions::East => {
                input[current_row][current_column] = '[';
            }
            Directions::West => {
                input[current_row][current_column] = ']';
            }
            _ => {
                panic!("Invalid S location");
            }
        },
        Directions::South => match starting_direction {
            Directions::South => {
                input[current_row][current_column] = '!';
            }
            Directions::East => {
                input[current_row][current_column] = '{';
            }
            Directions::West => {
                input[current_row][current_column] = '}';
            }
            _ => {
                panic!("Invalid S location");
            }
        },
        Directions::East => match starting_direction {
            Directions::East => {
                input[current_row][current_column] = '=';
            }
            Directions::North => {
                input[current_row][current_column] = '}';
            }
            Directions::South => {
                input[current_row][current_column] = ']';
            }
            _ => {
                panic!("Invalid S location");
            }
        },
        Directions::West => match starting_direction {
            Directions::West => {
                input[current_row][current_column] = '=';
            }
            Directions::North => {
                input[current_row][current_column] = '{';
            }
            Directions::South => {
                input[current_row][current_column] = '[';
            }
            _ => {
                panic!("Invalid S location");
            }
        },
    }

    count = 0;
    let collapse_to_wall = Regex::new(r"(\{=*])|(\[=*\})").unwrap();
    let collapse_to_nothing = Regex::new(r"[\[{]=*[\]}]").unwrap();
    let wall = Regex::new(r"![^!]*!").unwrap();
    for line in input {
        let current_line = line.into_iter().collect::<String>();
        let walls_collapsed = collapse_to_wall.replace_all(&current_line, "!");
        let empty_space_collapsed = collapse_to_nothing.replace_all(&walls_collapsed, "");

        for cap in wall.captures_iter(&empty_space_collapsed) {
            count += String::from(&cap[0]).len() - 2;
        }
    }

    println!("Part 2: {}", count);
}

#[derive(PartialEq, Debug, Clone)]
enum Directions {
    North,
    South,
    East,
    West,
}

fn get_next_location(
    row: usize,
    column: usize,
    direction_traveled: Directions,
    input: &mut Vec<Vec<char>>,
) -> (usize, usize, Directions) {
    match input[row][column] {
        '|' => {
            input[row][column] = '!';
            if direction_traveled == Directions::North {
                return (row - 1, column, Directions::North);
            }
            if direction_traveled == Directions::South {
                return (row + 1, column, Directions::South);
            }
            panic!("Invalid direction: traveling {:?} at |", direction_traveled);
        }
        '-' => {
            input[row][column] = '=';
            if direction_traveled == Directions::East {
                return (row, column + 1, Directions::East);
            }
            if direction_traveled == Directions::West {
                return (row, column - 1, Directions::West);
            }
            panic!("Invalid direction: traveling {:?} at -", direction_traveled);
        }
        '7' => {
            input[row][column] = ']';
            if direction_traveled == Directions::East {
                return (row + 1, column, Directions::South);
            }
            if direction_traveled == Directions::North {
                return (row, column - 1, Directions::West);
            }
            panic!("Invalid direction: traveling {:?} at 7", direction_traveled);
        }
        'J' => {
            input[row][column] = '}';
            if direction_traveled == Directions::East {
                return (row - 1, column, Directions::North);
            }
            if direction_traveled == Directions::South {
                return (row, column - 1, Directions::West);
            }
            panic!("Invalid direction: traveling {:?} at J", direction_traveled);
        }
        'L' => {
            input[row][column] = '{';
            if direction_traveled == Directions::West {
                return (row - 1, column, Directions::North);
            }
            if direction_traveled == Directions::South {
                return (row, column + 1, Directions::East);
            }
            panic!("Invalid direction: traveling {:?} at L", direction_traveled);
        }
        'F' => {
            input[row][column] = '[';
            if direction_traveled == Directions::West {
                return (row + 1, column, Directions::South);
            }
            if direction_traveled == Directions::North {
                return (row, column + 1, Directions::East);
            }
            panic!("Invalid direction: traveling {:?} at F", direction_traveled);
        }
        character => {
            panic!("Invalid pipe character: {}", character);
        }
    }
}

fn find_start(input: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No starting point found");
}

fn get_starting_direction(
    start_location: (usize, usize),
    input: &Vec<Vec<char>>,
) -> (usize, usize, Directions) {
    let north_of_start = input[start_location.0 - 1][start_location.1];
    let south_of_start = input[start_location.0 + 1][start_location.1];
    let east_of_start = input[start_location.0][start_location.1 + 1];

    if north_of_start == '|' || north_of_start == 'F' || north_of_start == '7' {
        (start_location.0 - 1, start_location.1, Directions::North)
    } else if south_of_start == '|' || south_of_start == 'L' || south_of_start == 'J' {
        (start_location.0 + 1, start_location.1, Directions::South)
    } else if east_of_start == '-' || east_of_start == '7' || east_of_start == 'J' {
        (start_location.0, start_location.1 + 1, Directions::East)
    } else {
        (start_location.0, start_location.1 - 1, Directions::West)
    }
}
