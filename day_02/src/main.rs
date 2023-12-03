use std::collections::HashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, line_ending,
    },
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}
#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

impl<'a> Game<'a> {

    fn is_valid(&self, limits: &HashMap<&str, u32>) -> Option<u32> {
        self.rounds.iter().all(|round| {
            round.iter().all(|shown_cube| {
                shown_cube.amount <= *limits
                    .get(shown_cube.color)
                    .expect("a valid color is always shown")
            })
        }).then_some(self.id.parse::<u32>().expect("should parse"))
    }

    fn power(&self) -> u32 {
        let minimum: HashMap<&str, u32> = HashMap::new();
        self.rounds
            .iter()
            .fold(minimum, |mut acc, round|{
                for cube in round.iter() {
                    acc.entry(cube.color)
                        .and_modify(|v| {
                            *v = (*v).max(cube.amount);
                        })
                        .or_insert(cube.amount);
                }
                acc
            })
            .values()
            .product()
        }
}

//3 blue
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = 
        separated_pair(complete::u32, tag(" "), alpha1)(input,)?;
    Ok((input, Cube {color, amount}))
}

// 3 blue, 4 red
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round),)(input)?;
    Ok((input, Game {id, rounds}))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = 
        separated_list1(line_ending, game)(input)?;
    
    Ok((input, games))
}


fn main() {
    let input = include_str!("./input.txt");
    let limits = HashMap::from([
        ("red",   12),
        ("green", 13),
        ("blue",  14 )]);
    let games = parse_games(input).expect("Parse MF");
    let part1 = games.1.iter().filter_map(|game| game.is_valid(&limits)).sum::<u32>();
    let part2 = games.1.iter().map(|game| game.power()).sum::<u32>();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    //dbg!(games);
}
