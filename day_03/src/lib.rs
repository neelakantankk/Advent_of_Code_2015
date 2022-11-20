use std::error::Error;
use std::collections::HashMap;

pub fn run(contents: String) -> Result<(),Box<dyn Error>> {

    let houses = parse_instructions_p1(&contents);
    let houses_visited = houses.keys().count();

    println!("Santa visited {} houses",houses_visited);
    Ok(())
}

fn parse_instructions_p1(contents: &str) -> HashMap<(i32,i32),u32> {
    let mut houses: HashMap<(i32,i32),u32> = HashMap::new();
    // Tracking houses on grid using (x,y) coordinates
    // x is East--West, y is N--S
    let mut current_pos = (0,0);
    houses.insert(current_pos,1);

    for c in contents.chars() {
        let new_pos = match c {
            '^' => (current_pos.0,current_pos.1+1),
            'v' => (current_pos.0, current_pos.1-1),
            '>' => (current_pos.0+1, current_pos.1),
            _ => (current_pos.0-1,current_pos.1),
        };
        current_pos = new_pos;
        houses.entry(current_pos).and_modify(|p| *p+=1).or_insert(1);
    }
    houses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(2,parse_instructions_p1(">").keys().count());
        assert_eq!(4,parse_instructions_p1("^>v<").keys().count());
        assert_eq!(2,parse_instructions_p1("^v^v^v^v^v").keys().count());
    }

}

