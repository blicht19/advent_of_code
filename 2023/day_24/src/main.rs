use library::{get_filename_arg, get_lines};
use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

static MIN: f32 = 200000000000000.;
static MAX: f32 = 400000000000000.;

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(&file_name);
    let mut part_one_stones = vec![];
    let mut part_two_stones = vec![];
    for line in lines {
        let line: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        let parts: Vec<_> = line.split("@").collect();
        let coords: Vec<_> = parts[0].split(",").collect();
        let velocities: Vec<_> = parts[1].split(",").collect();
        part_one_stones.push(PartOneStone::new(&coords, &velocities));
        part_two_stones.push(PartTwoStone::new(&coords, &velocities));
    }

    println!("Part 1: {}", part_one(&part_one_stones));
    println!("Part 2: {}", part_two(&part_two_stones));
}

/* Refactored based on HyperNeutrino's solution https://youtu.be/guOyA7Ijqgk?si=yqK6OEwMjJ7mXTrT */
fn part_one(stones: &Vec<PartOneStone>) -> u32 {
    let mut count = 0;

    for i in 0..stones.len() - 1 {
        for j in i..stones.len() {
            if stones[i].will_collide(&stones[j]) {
                count += 1;
            }
        }
    }

    count
}

/* No idea how to solve this without using some equation solving library, so based this on arthomnix's solution using z3.
* If you on this repo on Github just now that the devcontainer config will not work for part 2 as is because you need clang
* installed for z3 to build.
*/
fn part_two(stones: &Vec<PartTwoStone>) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let velocity_x = Int::new_const(&ctx, "velocity_x");
    let velocity_y = Int::new_const(&ctx, "velocity_y");
    let velocity_z = Int::new_const(&ctx, "velocity_z");

    for stone in stones {
        let stone_x = Int::from_i64(&ctx, stone.x);
        let stone_y = Int::from_i64(&ctx, stone.y);
        let stone_z = Int::from_i64(&ctx, stone.z);
        let stone_velocity_x = Int::from_i64(&ctx, stone.velocity_x);
        let stone_velocity_y = Int::from_i64(&ctx, stone.velocity_y);
        let stone_velocity_z = Int::from_i64(&ctx, stone.velocity_z);
        let t = Int::fresh_const(&ctx, "t");

        solver.assert(&(&stone_x + &stone_velocity_x * &t)._eq(&(&x + &velocity_x * &t)));
        solver.assert(&(&stone_y + &stone_velocity_y * &t)._eq(&(&y + &velocity_y * &t)));
        solver.assert(&(&stone_z + &stone_velocity_z * &t)._eq(&(&z + &velocity_z * &t)));
    }

    solver.check();

    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&x).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&y).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&z).unwrap().as_i64().unwrap();

    x + y + z
}

struct PartOneStone {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl PartOneStone {
    fn new(coords: &Vec<&str>, velocities: &Vec<&str>) -> Self {
        Self {
            x: coords[0].parse().unwrap(),
            y: coords[1].parse().unwrap(),
            velocity_x: velocities[0].parse().unwrap(),
            velocity_y: velocities[1].parse().unwrap(),
        }
    }

    fn will_collide(&self, other: &Self) -> bool {
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

struct PartTwoStone {
    x: i64,
    y: i64,
    z: i64,
    velocity_x: i64,
    velocity_y: i64,
    velocity_z: i64,
}

impl PartTwoStone {
    fn new(coords: &Vec<&str>, velocities: &Vec<&str>) -> Self {
        Self {
            x: coords[0].parse().unwrap(),
            y: coords[1].parse().unwrap(),
            z: coords[2].parse().unwrap(),
            velocity_x: velocities[0].parse().unwrap(),
            velocity_y: velocities[1].parse().unwrap(),
            velocity_z: velocities[2].parse().unwrap(),
        }
    }
}
