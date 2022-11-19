use std::env;
use std::error::Error;

pub fn get_final_floor(contents: &str) -> i32 {
    let count_open_parens = contents.chars().filter(|x| *x=='(').count();
    let count_closed_parens = contents.chars().filter(|x| *x ==')').count();

    (0 + count_open_parens - count_closed_parens).try_into().unwrap()

}

pub fn get_first_basement(contents: &str) -> Option<u32> {
    let mut current_floor = 0;

    for (index, c) in contents.chars().enumerate() {
        if c == '(' {
            current_floor +=1;
        } else if c == ')' {
            current_floor -=1;
        }

        if current_floor == -1 {
            return Some((index+1).try_into().unwrap());
        }
    }
    None
}

pub fn run(contents:String ) -> Result<(),Box<dyn Error>> {

    let part_num = env::var("PART_NUM").unwrap_or("0".to_string());
    
    if part_num == "1" {
        println!("Santa ends up at {}",get_final_floor(&contents));
    } else if part_num == "2" {
        println!("Santa is sent to the basement at character {}",
            get_first_basement(&contents).ok_or("No basement found")?);
    } else if part_num == "0" {
        println!("Santa ends up at {}",get_final_floor(&contents));
        println!("Santa is sent to the basement at character {}",
            get_first_basement(&contents).ok_or("No basement found")?);
    }
    Ok(())
}






