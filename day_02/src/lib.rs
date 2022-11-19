use std::error::Error;

#[derive(Debug)]
pub struct RectangleBox {
    length: u32,
    width: u32,
    height: u32,
    surface_area: u32,
    smallest_side: u32,
}

impl RectangleBox {
    pub fn new<'a>(mut dimensions: impl Iterator<Item = &'a str>) -> RectangleBox {
        let length = dimensions.next().unwrap().parse().unwrap();
        let width = dimensions.next().unwrap().parse().unwrap();
        let height = dimensions.next().unwrap().parse().unwrap();

        let surface_area = 2 * ((length * width) + (length * height) + (height * width));

        let smallest_side = smallest_side(length, width, height);

        RectangleBox {
            length,
            width,
            height,
            surface_area,
            smallest_side,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn surface_area(&self) -> u32 {
        self.surface_area
    }
    pub fn smallest_side(&self) -> u32 {
        self.smallest_side
    }
}

fn smallest_side(length: u32, width: u32, height: u32) -> u32 {
    let lw = length * width;
    let lh = length * height;
    let wh = width * height;

    let mut sides = vec![lw, lh, wh];

    sides.sort();

    sides[0]
}

fn get_total_wrapping_paper(boxes: &[RectangleBox]) -> u32 {
    let mut paper = 0;

    for a_box in boxes {
        paper += a_box.surface_area() + a_box.smallest_side();
    }
    paper
}

pub fn run(contents: String) -> Result<(),Box<dyn Error>> {

    let mut boxes : Vec<RectangleBox> = Vec::new();
    
    for line in contents.lines() {
        boxes.push(RectangleBox::new(line.split("x")));
    }

    let total_paper_needed = get_total_wrapping_paper(&boxes);
    println!("Total paper needed: {} sq. ft",total_paper_needed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let newbox = RectangleBox::new("2x3x4".split("x"));
        assert_eq!(58, get_total_wrapping_paper(&[newbox]));
    }
}
