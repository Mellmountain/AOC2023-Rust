fn main() {
    let input = include_str!("./input.txt").lines();
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    for line in input {
        let nums = line.split_whitespace().map(|nr| nr.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        numbers.push(nums);
    }
    
    let mut part1 = 0;
    let mut part2 = 0;
    for nums in numbers {
        part1 += find_last(&nums);
        part2 += find_first(&nums);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_last(nums: &[i64]) -> i64 {
    let mut answer = *nums.last().unwrap();
    let mut diffs = nums.windows(2).map(|pair| pair[1] - pair[0]).collect::<Vec<i64>>();
    answer += diffs.last().unwrap();

    loop {
        diffs = diffs.windows(2).map(|pair| pair[1] - pair[0]).collect::<Vec<i64>>();
        if diffs[0] == diffs[1] && diffs.iter().all(|num| *num == 0) {
            return answer;
        }
        answer += diffs.last().unwrap();
    }
}

fn find_first(nums: &[i64]) -> i64 {
    let mut answer = *nums.first().unwrap();
    let mut diffs = nums.windows(2).map(|pair| pair[1] - pair[0]).collect::<Vec<i64>>();
    answer -= diffs.first().unwrap();

    loop {
        diffs = diffs.windows(2).map(|pair| pair[1] - pair[0]).collect::<Vec<i64>>();
        if diffs[0] == diffs[1] && diffs.iter().all(|num| *num == 0){
            return answer;
        }
        answer -= diffs.first().unwrap();
    }
}

