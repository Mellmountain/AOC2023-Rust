//Time:        56     97     77     93
//Distance:   499   2210   1097   1440

//Time:      7  15   30
//Distance:  9  40  200


fn main() {
    println!("Hello, world!");
    //let time: Vec<i64> = vec![7, 15, 30];  // Sample
    //let dist: Vec<i64> = vec![9, 40, 200]; // Sample
    let time: Vec<i64> = vec![56, 97, 77, 93];
    let dist: Vec<i64> = vec![499, 2210, 1097, 1440];
    let time_p2: Vec<i64> = vec![56977793];
    let dist_p2: Vec<i64> = vec![499221010971440];
    let mut result: Vec<Vec<i64>>  = Vec::new();
    let mut result_p2: Vec<Vec<i64>>  = Vec::new();
    for t in time {
        let mut race: Vec<i64> = Vec::new();
        for i in 1..t {
            race.push(i *(t - i));
        }
        result.push(race);
    }
    for t in time_p2 {
        let mut race: Vec<i64> = Vec::new();
        for i in 1..t {
            race.push(i *(t - i));
        }
        result_p2.push(race);
    }
    let mut part1 = 1;
    for (idx, v) in result.iter().enumerate() {
        part1 *= v.iter().filter(|d| d > &&dist[idx]).count();
    }
    let mut part2 = 1;
    for (idx, v) in result_p2.iter().enumerate() {
        part2 *= v.iter().filter(|d| d > &&dist_p2[idx]).count();
    }
    println!("{}", part1);
    println!("{}", part2)

}
