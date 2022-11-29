use std::error::Error;

pub fn run(contents: String) -> Result<(),Box<dyn Error>> {
    let mut count = 1;
    for line in contents.lines() {
        if is_nice(line) {
            count += 1;
        }
    }
    println!("There are {} nice lines",count);
    Ok(())
}

fn is_nice(line: &str) -> bool {
    if contains_naughty_string(line) {
        return false;
    } else if !contains_repeat_char(line) {
        return false;
    } else if !contains_three_vowels(line) {
        return false;
    }
    true
}

fn contains_naughty_string(line: &str) -> bool {
    line.contains("ab") || line.contains("cd") 
        || line.contains("pq") || line.contains("xy")
}

fn contains_repeat_char(line: &str) -> bool {
    let mut prev_char = ' ';
    for c in line.chars() {
        if c == prev_char {
            return true;
        }
        prev_char = c;
    }
    false
}

fn contains_three_vowels(line: &str) -> bool {
    let count = line.matches(|c| "aeiou".contains(c)).count();
    count > 2
}
