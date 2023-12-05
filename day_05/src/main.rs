use std::ops::Range;
#[derive(Debug, Default)]
struct Almanac {
    seeds_part1: Vec<i64>,
    seeds_part2: Vec<Range<i64>>,
    maps: Vec<Mapping>,
}

#[derive(Default, Debug)]
struct Mapping {
    map: Vec<Description>,
}

impl Mapping {
    fn add_map(&mut self, dest: i64, src: i64, delta: i64, inp: String) {
        self.map.push(Description { 
            range: Range {
                start: src, end: src + delta
            }, delta: dest - src, _input: inp })
    }

    fn from_to(&self, value: i64) -> i64 {
        for desc in &self.map {
            if desc.range.contains(&value) {
                return value + desc.delta;
            }
        }
        value //Any source numbers that aren't mapped correspond to the same destination number.
    }
}

#[derive(Debug)]
struct Description {
    range: Range<i64>,
    delta: i64,
    _input: String
}


fn main() {
    let mut input = include_str!("./input.txt").lines();
    let mut almanac = Almanac::default();
    
    let (_, inp_seeds) = input.next().expect("seeds is first!").split_once(": ").unwrap();
    
    almanac.seeds_part1 = inp_seeds.split_whitespace().map(|seed| seed.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    for i in (0..almanac.seeds_part1.len()).step_by(2) {
        let start = almanac.seeds_part1[i];
        let length = almanac.seeds_part1[i+1]; 
        almanac.seeds_part2.push(start..start + length);
    }

    let mut new_map: Mapping = Mapping::default();
    for line in input {
        if line.len() == 0 { continue;}
        else if line.contains(":") {
            if !new_map.map.is_empty() {
                almanac.maps.push(new_map);
                new_map = Mapping::default();
            }
            continue;
        } else {
            let desc: Vec<i64> = line.split_whitespace().map(|nums| nums.parse::<i64>().unwrap()).collect();
            new_map.add_map(desc[0], desc[1],desc[2], line.to_string());
        }
    }
    if !new_map.map.is_empty() {
        almanac.maps.push(new_map);
    }

    let mut part1 = i64::MAX;
    for seed in almanac.seeds_part1 {
        let mut transform = seed;
        for map in &almanac.maps {
            transform = map.from_to(transform);
        }
        part1 = part1.min(transform);
    }

    let mut part2 = i64::MAX;
    for seeds in almanac.seeds_part2 {
        for seed in seeds {
            let mut transform = seed;
            for map in &almanac.maps {
                transform = map.from_to(transform);
            }
        part2 = part2.min(transform);
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

}
