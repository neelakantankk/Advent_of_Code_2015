extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
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
    let mut lights_on: HashSet<Point> = HashSet::new();
    for line in contents.lines() {
        lights_on = parse_line(line, lights_on);
    }
    println!("There are {} lights on", lights_on.len());
    Ok(())
}

fn parse_line(line: &str, lights_on: HashSet<Point>) -> HashSet<Point> {
    let mut lights_on = lights_on;
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
    if instruction == "toggle" {
        toggle(upper_left, lower_right, &mut lights_on);
    } else if instruction == "turn on" {
        turn_on(upper_left, lower_right, &mut lights_on);
    } else if instruction == "turn off" {
        turn_off(upper_left, lower_right, &mut lights_on);
    }
    lights_on
}

fn turn_on(upper_left: Point, lower_right: Point, lights_on: &mut HashSet<Point>) {
    let points = generate_points(upper_left, lower_right);
    for point in points {
        lights_on.insert(point);
    }
}

fn turn_off(upper_left: Point, lower_right: Point, lights_on: &mut HashSet<Point>) {
    let points = generate_points(upper_left, lower_right);
    for point in points {
        lights_on.remove(&point);
    }
}

fn toggle(upper_left: Point, lower_right: Point, lights_on: &mut HashSet<Point>) {
    let points = generate_points(upper_left, lower_right);
    for point in points {
        if lights_on.contains(&point) {
            lights_on.remove(&point);
        } else {
            lights_on.insert(point);
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
        lights_on = parse_line(line, lights_on);
        assert_eq!(1000 * 1000, lights_on.len());
    }
}
