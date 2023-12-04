use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Card {
    fn score(&self) -> i32 {
        let count = self.winning.intersection(&self.numbers).count() as u32;
        if count > 0 {
            i32::pow(2, count - 1)
        } else {
            0
        }
    }
}

fn main() {
    let input = include_str!("./input.txt").lines();
    let mut cards: Vec<Card> = Vec::new();
    for line in input {
        let (_, nums) = line.split_once(": ").unwrap();
        let (winno, chooseno) = nums.split_once(" | ").unwrap();

        let winning = winno.split_whitespace().map(|sno| sno.parse::<i32>().unwrap()).collect::<HashSet<i32>>();
        let numbers = chooseno.split_whitespace().map(|sno| sno.parse::<i32>().unwrap()).collect::<HashSet<i32>>();

        cards.push(Card {winning, numbers});
    }

    println!("Part 1: {}", cards.iter().map(Card::score).sum::<i32>());
}
