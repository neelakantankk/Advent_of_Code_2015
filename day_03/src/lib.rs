use std::error::Error;
use std::collections::HashSet;

pub fn run(contents: String) -> Result<(),Box<dyn Error>> {

    let houses = parse_instructions_p1(&contents);
    let houses_visited = houses.len();

    println!("Santa visited {} houses",houses_visited);

    let houses = parse_instructions_p2(&contents);
    let houses_visited = houses.len();

    println!("Houses visited: {}", houses_visited);
    Ok(())
}

fn parse_instructions_p1(contents: &str) -> HashSet<(i32,i32)> {
    let mut houses: HashSet<(i32,i32)> = HashSet::new();
    // Tracking houses on grid using (x,y) coordinates
    // x is East--West, y is N--S
    let mut current_pos = (0,0);
    houses.insert(current_pos);

    for c in contents.chars() {
        current_pos = get_new_pos(current_pos,c);
        houses.insert(current_pos);
    }
    houses
}

fn parse_instructions_p2(contents: &str) -> HashSet<(i32,i32)> {
    let mut houses : HashSet<(i32,i32)> = HashSet::new();
    let mut santa_pos = (0,0);
    let mut robo_pos = (0,0);

    houses.insert(santa_pos);
    houses.insert(robo_pos);

    for (index,c) in contents.char_indices() {
        if index % 2 == 0 {
            santa_pos = get_new_pos(santa_pos,c);
            houses.insert(santa_pos);
        } else {
            robo_pos = get_new_pos(robo_pos,c);
            houses.insert(robo_pos);
        }
    }
    houses
}

fn get_new_pos(current_pos: (i32,i32),c:char) -> (i32,i32) {
     match c {
        '^' => (current_pos.0,current_pos.1+1),
        'v' => (current_pos.0, current_pos.1-1),
        '>' => (current_pos.0+1, current_pos.1),
        _ => (current_pos.0-1,current_pos.1),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(2,parse_instructions_p1(">").len());
        assert_eq!(4,parse_instructions_p1("^>v<").len());
        assert_eq!(2,parse_instructions_p1("^v^v^v^v^v").len());
    }

    #[test]
    fn test_part_two() {
        assert_eq!(3, parse_instructions_p2("^v").len());
        assert_eq!(11, parse_instructions_p2("^v^v^v^v^v").len());
    }
}

