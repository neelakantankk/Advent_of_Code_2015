extern crate fancy_regex;
use fancy_regex::Regex;
use std::env;
use std::error::Error;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    let var = env::var("PART").unwrap_or_else(|_| 0.to_string());
    if var == "1" {
        for line in contents.lines() {
            if is_nice_p1(line) {
                count += 1;
            }
        }
    } else if var == "2" {
        for line in contents.lines() {
            if is_nice_p2(line) {
                count += 1;
            }
        }
    } else {
        panic!("Specify PART=1 or PART=2");
    }

    println!("There are {} nice lines", count);
    Ok(())
}

fn is_nice_p1(line: &str) -> bool {
    !contains_naughty_string(line) && contains_repeat_char(line) && contains_three_vowels(line)
}

fn is_nice_p2(line: &str) -> bool {
    contains_distinct_pairs(line) && contains_separated_repeat(line)
}

fn contains_distinct_pairs(line: &str) -> bool {
    let re = Regex::new(r"(\w)(\w).*\1\2").unwrap();
    re.is_match(line).unwrap()
}

fn contains_separated_repeat(line: &str) -> bool {
    let re = Regex::new(r"(\w)\w\1").unwrap();
    re.is_match(line).unwrap()
}

fn contains_naughty_string(line: &str) -> bool {
    line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
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
mod tests {
    use super::*;

    #[test]
    fn test_naughty_string() {
        assert!(contains_naughty_string("plabqy"), "plabqy is nice");
        assert!(contains_naughty_string("acdwrt"), "acdwrt is nice");
        assert!(contains_naughty_string("lmnopq"), "lmnopq is nice");
        assert!(contains_naughty_string("uvwxyzap"), "uvwxyzap is nice");
        assert!(!contains_naughty_string("acbdpxqy"), "acbdpxqy is naughty");
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

    #[test]
    fn test_distinct_pair() {
        assert!(contains_distinct_pairs("xyxy"));
        assert!(contains_distinct_pairs("aabcdefgaa"));
        assert!(!contains_distinct_pairs("aaa"));
    }

    #[test]
    fn test_separated_repeat() {
        assert!(contains_separated_repeat("xyx"));
        assert!(contains_separated_repeat("abcdefeghi"));
        assert!(contains_separated_repeat("aaa"));
        assert!(!contains_separated_repeat("aab"));
        assert!(!contains_separated_repeat("abba"));
    }

    #[test]
    fn test_p1() {
        assert!(is_nice_p1("ugknbfddgicrmopn"));
        assert!(is_nice_p1("aaa"));
        assert!(!is_nice_p1("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_p2() {
        assert!(is_nice_p2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_p2("xxyxx"));
        assert!(!is_nice_p2("uurcxstgmygtbstg"));
        assert!(!is_nice_p2("ieodomkazucvgmuy"));
    }
}
