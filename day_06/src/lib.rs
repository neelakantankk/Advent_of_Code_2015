extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
    pub fn from_vec(xy: Vec<u32>) -> Point {
        if xy.len() != 2 {
            panic!("Could not make point from {:?}", xy);
        }
        Point { x: xy[0], y: xy[1] }
    }
}

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let pnum = env::var("PART");
    match pnum {
        Ok(val) => {
            if val == "1" {
                let mut lights_on: HashSet<Point> = HashSet::new();
                for line in contents.lines() {
                    lights_on = part_one(line, lights_on);
                }
                println!("There are {} lights on", lights_on.len());
            } else if val == "2" {
                let mut lights_on: HashMap<Point, u32> = HashMap::with_capacity(1000 * 1000);
                for line in contents.lines() {
                    lights_on = part_two(line, lights_on);
                }
                println!("The brightness is {}", lights_on.values().sum::<u32>());
            }
        }
        Err(_) => {
            panic!("Must Specify PART=1 or PART=2");
        }
    };

    Ok(())
}

fn part_one(line: &str, lights_on: HashSet<Point>) -> HashSet<Point> {
    let mut lights_on = lights_on;
    let (instruction, upper_left, lower_right) = parse_line(line);
    if instruction == "toggle" {
        part_one::toggle(upper_left, lower_right, &mut lights_on);
    } else if instruction == "turn on" {
        part_one::turn_on(upper_left, lower_right, &mut lights_on);
    } else if instruction == "turn off" {
        part_one::turn_off(upper_left, lower_right, &mut lights_on);
    }
    lights_on
}

fn part_two(line: &str, lights_on: HashMap<Point, u32>) -> HashMap<Point, u32> {
    let mut lights_on = lights_on;
    let (instruction, upper_left, lower_right) = parse_line(line);
    if instruction == "toggle" {
        part_two::toggle(upper_left, lower_right, &mut lights_on);
    } else if instruction == "turn on" {
        part_two::turn_on(upper_left, lower_right, &mut lights_on);
    } else if instruction == "turn off" {
        part_two::turn_off(upper_left, lower_right, &mut lights_on);
    }
    lights_on
}

fn parse_line(line: &str) -> (&str, Point, Point) {
    let re = Regex::new(
        r"(?P<instruction>toggle|turn on|turn off) (?P<up_l>\d+,\d+) through (?P<low_r>\d+,\d+)",
    )
    .unwrap();
    let caps = re.captures(line).unwrap();
    let instruction = caps.name("instruction").unwrap().as_str();
    let upper_left = Point::from_vec(
        caps.name("up_l")
            .unwrap()
            .as_str()
            .split_terminator(',')
            .map(|d| d.parse::<u32>().unwrap())
            .collect::<Vec<u32>>(),
    );
    let lower_right = Point::from_vec(
        caps.name("low_r")
            .unwrap()
            .as_str()
            .split_terminator(',')
            .map(|d| d.parse::<u32>().unwrap())
            .collect::<Vec<u32>>(),
    );
    (instruction, upper_left, lower_right)
}

mod part_one {
    use super::*;

    pub fn turn_on(upper_left: Point, lower_right: Point, lights_on: &mut HashSet<Point>) {
        let points = generate_points(upper_left, lower_right);
        for point in points {
            lights_on.insert(point);
        }
    }

    pub fn turn_off(upper_left: Point, lower_right: Point, lights_on: &mut HashSet<Point>) {
        let points = generate_points(upper_left, lower_right);
        for point in points {
            lights_on.remove(&point);
        }
    }

    pub fn toggle(upper_left: Point, lower_right: Point, lights_on: &mut HashSet<Point>) {
        let points = generate_points(upper_left, lower_right);
        for point in points {
            if lights_on.contains(&point) {
                lights_on.remove(&point);
            } else {
                lights_on.insert(point);
            }
        }
    }
}
mod part_two {
    use super::*;

    pub fn turn_on(upper_left: Point, lower_right: Point, lights_on: &mut HashMap<Point, u32>) {
        let points = generate_points(upper_left, lower_right);
        for point in points {
            lights_on.entry(point).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    pub fn turn_off(upper_left: Point, lower_right: Point, lights_on: &mut HashMap<Point, u32>) {
        let points = generate_points(upper_left, lower_right);
        for point in points {
            lights_on
                .entry(point)
                .and_modify(|v| {
                    if *v > 0 {
                        *v -= 1
                    }
                })
                .or_insert(0);
        }
    }

    pub fn toggle(upper_left: Point, lower_right: Point, lights_on: &mut HashMap<Point, u32>) {
        let points = generate_points(upper_left, lower_right);
        for point in points {
            lights_on.entry(point).and_modify(|v| *v += 2).or_insert(2);
        }
    }
}

fn generate_points(upper_left: Point, lower_right: Point) -> Vec<Point> {
    let mut v: Vec<Point> = Vec::new();
    for i in upper_left.x..lower_right.x + 1 {
        for j in upper_left.y..lower_right.y + 1 {
            v.push(Point::new(i, j));
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_on() {
        let line = "turn on 0,0 through 999,999";
        let mut lights_on: HashSet<Point> = HashSet::new();
        lights_on = part_one(line, lights_on);
        assert_eq!(1000 * 1000, lights_on.len());
    }
    #[test]
    fn test_turn_on_part_two() {
        let line = "turn on 0,0 through 0,0";
        let mut lights_on: HashMap<Point, u32> = HashMap::new();
        lights_on = part_two(line, lights_on);
        assert_eq!(1 as u32, lights_on.values().sum());
    }
}
