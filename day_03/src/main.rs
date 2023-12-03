use std::collections::HashSet;

#[derive(Default)]
struct Day03 {
    numbers: Vec<PartNumber>,
    symbols: HashSet<(i32, i32)>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }
}
fn main() {
    let input = include_str!("./input.txt").lines();
    let mut day03 = Day03::new();
    let mut curr_part: Option<PartNumber> = None;
    for (y, line) in input.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if let Some(ref mut num) = curr_part {
                    num.add_digit(x as i32, y as i32, c);
                } else {
                    curr_part = Some(PartNumber::new(x as i32, y as i32, c));
                }
            } else {
                if let Some(num) = curr_part.take() {
                    day03.numbers.push(num);
                }
                if c != '.' {
                    day03.symbols.insert((x as i32, y as i32));
                }
            }
        }
    }
    let part1 = day03.numbers
                            .iter()
                            .filter(|part| part.is_valid(&day03.symbols))
                            .map(PartNumber::number)
                            .sum::<i32>();
    println!("{}", part1);

}
#[derive(Debug)]
struct PartNumber {
    value: i32,
    points: HashSet<(i32, i32)>,
}

impl PartNumber {
    fn new(x: i32, y: i32, c: char) -> Self {
        let points = HashSet::from([
            (x - 1, y), (x - 1, y - 1), (x - 1, y + 1), //Behind
            (x, y + 1), (x, y - 1),                     //Up-down
            (x + 1, y), (x + 1, y - 1), (x + 1, y + 1)  //Ahead
        ]);
        Self {
            value: (c as u8 - b'0') as i32,
            points
        }
    }
    
    fn add_digit(&mut self, x: i32, y: i32, c: char) {
        self.value = self.value * 10 + (c as u8 - b'0') as i32;
        self.points.extend([(x + 1, y), (x + 1, y - 1), (x + 1, y + 1)]);
    }
    
    fn is_valid(&self, symbols: &HashSet<(i32, i32)>) -> bool {
        self.points.intersection(&symbols).next().is_some()
    }
   
    fn number(&self) -> i32 {
        self.value
    }
}