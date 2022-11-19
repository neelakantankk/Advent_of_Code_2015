use std::error::Error;

#[derive(Debug)]
pub struct RectangleBox {
    length: u32,
    width: u32,
    height: u32,
    surface_area: u32,
    smallest_area: u32,
    smallest_perimeter: u32,
    volume: u32,
}

impl RectangleBox {
    pub fn new<'a>(mut dimensions: impl Iterator<Item = &'a str>) -> RectangleBox {
        let length = dimensions.next().unwrap().parse().unwrap();
        let width = dimensions.next().unwrap().parse().unwrap();
        let height = dimensions.next().unwrap().parse().unwrap();

        let surface_area = 2 * ((length * width) + (length * height) + (height * width));

        let smallest_perimeter = smallest_perimeter(length, width, height);

        let smallest_area = smallest_area(length, width, height);

        let volume = length * width * height;

        RectangleBox {
            length,
            width,
            height,
            surface_area,
            smallest_area,
            smallest_perimeter,
            volume,
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
    pub fn smallest_area(&self) -> u32 {
        self.smallest_area
    }
    pub fn smallest_perimeter(&self) -> u32 {
        self.smallest_perimeter
    }
    pub fn volume(&self) -> u32 {
        self.volume
    }
}

fn smallest_area(length: u32, width: u32, height: u32) -> u32 {
    let lw = length * width;
    let lh = length * height;
    let wh = width * height;

    let mut sides = vec![lw, lh, wh];

    sides.sort();

    sides[0]
}

fn smallest_perimeter(length: u32, width: u32, height: u32) -> u32 {
    let lw = 2 * (length + width);
    let lh = 2 * (length + height);
    let wh = 2 * (width + height);

    let mut sides = vec![lw, lh, wh];
    sides.sort();
    sides[0]
}

fn get_total_wrapping_paper(boxes: &[RectangleBox]) -> u32 {
    let mut paper = 0;

    for a_box in boxes {
        paper += a_box.surface_area() + a_box.smallest_area();
    }
    paper
}

fn get_total_ribbon(boxes: &[RectangleBox]) -> u32 {
    let mut ribbon = 0;

    for a_box in boxes {
        ribbon += a_box.volume() + a_box.smallest_perimeter();
    }
    ribbon
}

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let mut boxes: Vec<RectangleBox> = Vec::new();

    for line in contents.lines() {
        boxes.push(RectangleBox::new(line.split('x')));
    }

    let total_paper_needed = get_total_wrapping_paper(&boxes);
    println!("Total paper needed: {} sq. ft", total_paper_needed);

    let total_ribbon_needed = get_total_ribbon(&boxes);
    println!("Total ribbon needed: {} ft", total_ribbon_needed);

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

    #[test]
    fn test_part_two() {
        let newbox = RectangleBox::new("2x3x4".split("x"));
        assert_eq!(34, get_total_ribbon(&[newbox]));
    }
}
