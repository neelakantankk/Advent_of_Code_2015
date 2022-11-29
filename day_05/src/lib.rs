use std::error::Error;

pub fn run(contents: String) -> Result<(),Box<dyn Error>> {
    let mut count = 0;
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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_naughty_string() {
        assert!(contains_naughty_string("plabqy"),"plabqy is nice");
        assert!(contains_naughty_string("acdwrt"),"acdwrt is nice");
        assert!(contains_naughty_string("lmnopq"),"lmnopq is nice");
        assert!(contains_naughty_string("uvwxyzap"),"uvwxyzap is nice");
        assert!(!contains_naughty_string("acbdpxqy"),"acbdpxqy is naughty");
    }

    #[test]
    fn test_three_vowels() {
        assert!(contains_three_vowels("aei"));
        assert!(contains_three_vowels("xazegov"));
        assert!(contains_three_vowels("aeiouaeiouaeiou"));
        assert!(!contains_three_vowels("aeb"));
        assert!(!contains_three_vowels("plsjyt"));
    }

    #[test]
    fn test_repeat_char() {
        assert!(contains_repeat_char("xx"));
        assert!(contains_repeat_char("abcdde"));
        assert!(contains_repeat_char("aabbccdd"));
        assert!(!contains_repeat_char("abcdef"));
        assert!(!contains_repeat_char("abcdabcdefe"));
    }
}
