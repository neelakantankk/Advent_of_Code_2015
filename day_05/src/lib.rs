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
    true
}
