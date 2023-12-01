fn main() {
    let mut input = include_str!("./sample.txt").lines();
    let mut sum = 0;
    for line in input {
        sum += parse_calibration(line);
    }
    println!("{}", sum);
}

fn parse_calibration(input: &str) -> u32 {
    let mut ten = 0;
    let mut one = 0;
    for c in input.chars() {
        if c.is_numeric() {
            ten = char::to_digit(c, 10).unwrap();
            break;
        }
    }
    for c in input.chars().rev() {
        if c.is_numeric() {
            one = char::to_digit(c, 10).unwrap();
            break;
        }
    }
    return (ten + one);
}
