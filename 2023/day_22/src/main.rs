use std::collections::HashSet;

use library::{get_filename_arg, get_lines};
use priority_queue::PriorityQueue;

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(&file_name);
    let mut bricks = vec![];

    for line in lines {
        let parts: Vec<_> = line.split("~").collect();
        let (a, b, c) = get_graph_coordinates(parts[0]);
        let (x, y, z) = get_graph_coordinates(parts[1]);
        bricks.push(Brick {
            x_range: (a, x),
            y_range: (b, y),
            z_range: (c, z),
            supporting_indices: vec![],
            supported_by_indices: vec![],
        });
    }
    bricks.sort_by_key(|brick| brick.z_range.0);
    bricks = drop_bricks(bricks);

    println!("Part 1: {}", part_one(&bricks));
    println!("Part 2: {}", part_two(&bricks));
}

fn part_one(bricks: &Vec<Brick>) -> usize {
    let mut sum = 0;

    'bricks: for i in 0..bricks.len() {
        for &index in bricks[i].supporting_indices.iter() {
            if bricks[index].supported_by_indices.len() == 1 {
                continue 'bricks;
            }
        }
        sum += 1;
    }

    sum
}

fn part_two(bricks: &Vec<Brick>) -> usize {
    let mut sum = 0;
    for i in (0..bricks.len()).rev() {
        let mut queue = PriorityQueue::new();
        queue.push(i, bricks[i].z_range.1);

        let mut fallen = HashSet::new();
        while let Some((index, _)) = queue.pop() {
            fallen.insert(index);

            'supporting: for supporting_index in &bricks[index].supporting_indices {
                for supported_by_index in &bricks[*supporting_index].supported_by_indices {
                    if !fallen.contains(supported_by_index) {
                        continue 'supporting;
                    }
                }
                queue.push(*supporting_index, bricks[*supporting_index].z_range.1);
                sum += 1;
            }
        }
    }
    sum
}

fn drop_bricks(bricks: Vec<Brick>) -> Vec<Brick> {
    let mut fallen_bricks: Vec<Brick> = vec![];

    for mut brick in bricks {
        loop {
            let mut can_drop = true;

            for i in (0..fallen_bricks.len()).rev() {
                if fallen_bricks[i].supports(&brick) {
                    can_drop = false;
                    let length = fallen_bricks.len();
                    fallen_bricks[i].supporting_indices.push(length);
                    brick.supported_by_indices.push(i);
                }
            }

            if !can_drop || brick.is_hitting_bottom() {
                break;
            }

            brick.drop();
        }

        fallen_bricks.push(brick);
    }

    fallen_bricks
}

fn get_graph_coordinates(coordinates: &str) -> (i32, i32, i32) {
    let coordinate_strings: Vec<_> = coordinates.split(",").collect();
    (
        coordinate_strings[0].parse().unwrap(),
        coordinate_strings[1].parse().unwrap(),
        coordinate_strings[2].parse().unwrap(),
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Brick {
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
    supporting_indices: Vec<usize>,
    supported_by_indices: Vec<usize>,
}

impl Brick {
    // intersects based on https://stackoverflow.com/a/30160064
    fn intersects(&self, other: &Brick) -> bool {
        let (x1, x2) = self.x_range;
        let (y1, y2) = self.y_range;
        let (x3, x4) = other.x_range;
        let (y3, y4) = other.y_range;

        let f1 = rotation_direction(x1, y1, x2, y2, x4, y4);
        let f2 = rotation_direction(x1, y1, x2, y2, x3, y3);
        let f3 = rotation_direction(x1, y1, x3, y3, x4, y4);
        let f4 = rotation_direction(x2, y2, x3, y3, x4, y4);

        if f1 != f2 && f3 != f4 {
            return true;
        }

        f1 == 0
            && f2 == 0
            && f3 == 0
            && f4 == 0
            && (contains_segment(x1, y1, x2, y2, x3, y3)
                || contains_segment(x1, y1, x2, y2, x4, y4)
                || contains_segment(x3, y3, x4, y4, x1, y1)
                || contains_segment(x3, y3, x4, y4, x2, y2))
    }
    fn supports(&self, other: &Brick) -> bool {
        other.z_range.0 == self.z_range.1 + 1 && self.intersects(other)
    }
    fn is_hitting_bottom(&self) -> bool {
        self.z_range.0 == 1
    }
    fn drop(&mut self) {
        self.z_range.0 -= 1;
        self.z_range.1 -= 1;
    }
}

// intersects based on https://stackoverflow.com/a/30160064

fn contains_segment(x1: i32, y1: i32, x2: i32, y2: i32, sx: i32, sy: i32) -> bool {
    (x1 < x2 && x1 < sx && sx < x2)
        || (x2 < x1 && x2 < sx && sx < x1)
        || (y1 < y2 && y1 < sy && sy < y2)
        || (y2 < y1 && y2 < sy && sy < y1)
        || (x1 == sx && y1 == sy || x2 == sx && y2 == sy)
}

fn rotation_direction(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> i32 {
    if ((y3 - y1) * (x2 - x1)) > ((y2 - y1) * (x3 - x1)) {
        return 1;
    } else if ((y3 - y1) * (x2 - x1)) == ((y2 - y1) * (x3 - x1)) {
        return 0;
    }
    -1
}
