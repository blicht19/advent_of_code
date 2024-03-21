use library::{get_filename_arg, get_lines};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    let input = get_lines(&file_name);

    println!("Part 1: {}", get_capacity(&get_part_one_plan(&input)));
    println!("Part 2: {}", get_capacity(&get_part_two_plan(&input)));
}

struct Instruction {
    direction: String,
    distance: i64,
}

struct Coordinate {
    x: i64,
    y: i64,
}

fn get_capacity(plan: &Vec<Instruction>) -> i64 {
    let vertices = get_vertices(plan);
    let perimeter = get_perimeter(plan);

    shoelace(vertices) + perimeter / 2 + 1
}

fn get_part_one_plan(input: &Vec<String>) -> Vec<Instruction> {
    let mut plan = vec![];

    for line in input {
        let sections: Vec<&str> = line.split(" ").collect();
        plan.push(Instruction {
            direction: String::from(sections[0]),
            distance: sections[1]
                .parse::<i64>()
                .expect("Failed to parse distance into number"),
        });
    }

    plan
}

fn get_part_two_plan(input: &Vec<String>) -> Vec<Instruction> {
    let re: Regex = Regex::new(r"[()]").unwrap();
    let mut plan = vec![];
    for line in input {
        let hex: Vec<char> = re
            .replace_all(line.split(" ").collect::<Vec<&str>>()[2], "")
            .chars()
            .collect();
        let direction = String::from(*hex.last().unwrap());
        let distance =
            i64::from_str_radix(&hex[1..hex.len() - 1].into_iter().collect::<String>(), 16)
                .expect("Failed to parse hex string into number");
        plan.push(Instruction {
            direction,
            distance,
        });
    }

    plan
}

fn get_perimeter(plan: &Vec<Instruction>) -> i64 {
    plan.iter().map(|instruction| instruction.distance).sum()
}

fn get_vertices(plan: &Vec<Instruction>) -> Vec<Coordinate> {
    let mut x = 0;
    let mut y = 0;
    let mut vertices = vec![Coordinate { x: 0, y: 0 }];

    for instruction in plan {
        match instruction.direction.as_str() {
            "U" | "3" => y += instruction.distance,
            "D" | "1" => y -= instruction.distance,
            "R" | "0" => x += instruction.distance,
            "L" | "2" => x -= instruction.distance,
            x => panic!("Invalid direction {}", x),
        }
        vertices.push(Coordinate { x, y });
    }

    vertices.pop();

    vertices
}

fn shoelace(vertices: Vec<Coordinate>) -> i64 {
    let mut xy = 0;
    let mut yx = 0;
    for i in 0..vertices.len() - 1 {
        xy += vertices[i].x * vertices[i + 1].y;
        yx += vertices[i].y * vertices[i + 1].x;
    }

    xy += vertices.last().unwrap().x * vertices.first().unwrap().y;
    yx += vertices.last().unwrap().y * vertices.first().unwrap().x;

    (xy - yx).abs() / 2
}
