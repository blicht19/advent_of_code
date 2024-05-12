use library::{get_filename_arg, get_lines};

static MIN: f32 = 200000000000000.;
static MAX: f32 = 400000000000000.;

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(&file_name);
    let mut stones = vec![];
    for line in lines {
        let line: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        let parts: Vec<_> = line.split("@").collect();
        let coords: Vec<_> = parts[0].split(",").collect();
        let velocities: Vec<_> = parts[1].split(",").collect();
        stones.push(Stone::new(
            coords[0].parse().unwrap(),
            coords[1].parse().unwrap(),
            coords[2].parse().unwrap(),
            velocities[0].parse().unwrap(),
            velocities[1].parse().unwrap(),
            velocities[2].parse().unwrap(),
        ));
    }

    println!("Part 1: {}", part_one(&stones));
}

fn part_one(stones: &Vec<Stone>) -> u32 {
    let mut count = 0;

    for i in 0..stones.len() - 1 {
        for j in i..stones.len() {
            if stones[i].will_collide_two_dimensions(&stones[j]) {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug)]
struct Stone {
    x: f32,
    y: f32,
    z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    two_dimensional_slope: f32,
}

impl Stone {
    fn new(x: f32, y: f32, z: f32, velocity_x: f32, velocity_y: f32, velocity_z: f32) -> Self {
        Self {
            x,
            y,
            z,
            velocity_x,
            velocity_y,
            velocity_z,
            two_dimensional_slope: velocity_y / velocity_x,
        }
    }

    fn will_collide_two_dimensions(&self, other: &Self) -> bool {
        let denominator = self.velocity_y * -other.velocity_x - other.velocity_y * -self.velocity_x;

        if denominator == 0. {
            return false;
        }

        let c1 = self.velocity_y * self.x - self.velocity_x * self.y;
        let c2 = other.velocity_y * other.x - other.velocity_x * other.y;

        let x_intersect = (c1 * -other.velocity_x - c2 * -self.velocity_x) / denominator;
        if x_intersect < MIN || x_intersect > MAX {
            return false;
        }

        let y_intersect = (c2 * self.velocity_y - c1 * other.velocity_y) / denominator;
        if y_intersect < MIN || y_intersect > MAX {
            return false;
        }

        self.point_is_in_future(x_intersect, y_intersect)
            && other.point_is_in_future(x_intersect, y_intersect)
    }

    fn point_is_in_future(&self, x: f32, y: f32) -> bool {
        ((x - self.x).is_sign_positive() == self.velocity_x.is_sign_positive())
            && ((y - self.y).is_sign_positive() == self.velocity_y.is_sign_positive())
    }
}
