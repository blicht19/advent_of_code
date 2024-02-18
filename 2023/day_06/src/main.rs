use std::u64::MAX;

use lazy_static::lazy_static;
use library::{get_filename_arg, get_lines};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(file_name.as_str());
    let part_one_times = get_numbers(&lines[0]);
    let part_two_distances = get_numbers(&lines[1]);
    let part_one_races = get_boat_races(part_one_times, part_two_distances);

    let part_one = get_part_one_product(part_one_races);
    println!("Part 1: {}", part_one);

    let part_two_race = BoatRace {
        time: get_single_number(&lines[0]),
        distance: get_single_number(&lines[1]),
    };
    let part_two = get_win_count(part_two_race);
    println!("Part 2: {}", part_two);
}

fn distance(time: u64, max_time: u64) -> u64 {
    return (max_time - time) * time;
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\d+").unwrap();
}

fn get_numbers(line: &String) -> Vec<u64> {
    REGEX
        .find_iter(line)
        .map(|n| {
            n.as_str()
                .parse()
                .expect("Could not parse string to number")
        })
        .collect()
}

fn get_single_number(line: &String) -> u64 {
    REGEX
        .find_iter(line)
        .map(|n| n.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("Failed to parse line into number")
}

struct BoatRace {
    time: u64,
    distance: u64,
}

fn get_boat_races(times: Vec<u64>, distances: Vec<u64>) -> Vec<BoatRace> {
    assert_eq!(times.len(), distances.len());

    let mut boat_races: Vec<BoatRace> = vec![];
    for i in 0..times.len() {
        boat_races.push(BoatRace {
            time: times[i],
            distance: distances[i],
        });
    }

    boat_races
}

fn get_win_count(boat_race: BoatRace) -> u64 {
    let mut min = MAX;
    let mut max = 0;

    for i in 0..boat_race.time {
        if distance(i, boat_race.time) > boat_race.distance {
            if min == MAX {
                min = i;
            }
        } else if min != MAX {
            max = i;
            break;
        }
    }

    max - min
}

fn get_part_one_product(races: Vec<BoatRace>) -> u64 {
    let mut product = 1;
    for race in races {
        let ways = get_win_count(race);
        product *= ways;
    }

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance(0, 7), 0);
        assert_eq!(distance(1, 7), 6);
        assert_eq!(distance(2, 7), 10);
        assert_eq!(distance(3, 7), 12);
        assert_eq!(distance(4, 7), 12);
        assert_eq!(distance(5, 7), 10);
        assert_eq!(distance(6, 7), 6);
        assert_eq!(distance(7, 7), 0);
    }

    #[test]
    fn test_get_numbers() {
        let times_string = String::from("Time:      7  15   30");
        let times = get_numbers(&times_string);

        assert_eq!(times.len(), 3);
        assert_eq!(times[0], 7);
        assert_eq!(times[1], 15);
        assert_eq!(times[2], 30);
    }

    #[test]
    fn test_get_boat_races() {
        let times = vec![0, 1];
        let distances = vec![2, 3];
        let boat_races = get_boat_races(times, distances);

        assert_eq!(boat_races.len(), 2);
        assert_eq!(boat_races[0].time, 0);
        assert_eq!(boat_races[0].distance, 2);
        assert_eq!(boat_races[1].time, 1);
        assert_eq!(boat_races[1].distance, 3);
    }

    #[test]
    fn test_get_win_count() {
        assert_eq!(
            get_win_count(BoatRace {
                time: 7,
                distance: 9
            }),
            4
        );
        assert_eq!(
            get_win_count(BoatRace {
                time: 15,
                distance: 40
            }),
            8
        );
        assert_eq!(
            get_win_count(BoatRace {
                time: 30,
                distance: 200
            }),
            9
        );
    }

    #[test]
    fn test_get_single_number() {
        assert_eq!(
            get_single_number(&String::from("Time:      7  15   30")),
            71530
        );
    }
}
