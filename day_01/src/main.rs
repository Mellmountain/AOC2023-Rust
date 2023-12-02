fn main() {
    let input = include_str!("./input.txt").lines();
    let input2 = include_str!("./input.txt").lines();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input {
        part1 += parse_calibration(line);
    }
    for line in input2 {
        part2 += parse_calibration_2(line);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

}

fn parse_calibration(input: &str) -> u32 {
    let mut tens = 0;
    let mut unit = 0;
    for c in input.chars() {
        if c.is_numeric() {
            tens = 10 * char::to_digit(c, 10).unwrap();
            break;
        }
    }
    for c in input.chars().rev() {
        if c.is_numeric() {
            unit = char::to_digit(c, 10).unwrap();
            break;
        }
    }
    return tens + unit;
}

fn parse_calibration_2(input: &str) -> u32  {
    let mut result = input.to_string(); 
    result = result.replace("one", "o1e");
    result = result.replace("two", "t2o");
    result = result.replace("three", "t3e");    
    result = result.replace("four", "f4r");    
    result = result.replace("five", "f5e");    
    result = result.replace("six", "s6x");    
    result = result.replace("seven", "s7n");    
    result = result.replace("eight", "e8t");    
    result = result.replace("nine", "n9e");
    
    println!("{}->{}->{}", input, result, parse_calibration(&result));
    return parse_calibration(&result);
}
