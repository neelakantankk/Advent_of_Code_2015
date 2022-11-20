extern crate md5;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let secret = "yzbqklnj";

    let smallest = get_smallest_number(&secret);
    println!("The smallest number is {}", smallest);

    Ok(())
}

fn get_smallest_number(secret: &str) -> u32 {
    let mut counter = 0;
    loop {
        let message = format!("{}{}", secret, counter);
        let digest = md5::compute(message);
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 16 {
            return counter;
        }
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(609043, get_smallest_number("abcdef"));
        assert_eq!(1048970, get_smallest_number("pqrstuv"));
    }
}
