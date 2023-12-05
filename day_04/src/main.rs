use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Card {
    fn score(&self) -> i32 {
        let count = self.matches() as u32;
        if count > 0 {
            i32::pow(2, count - 1)
        } else {
            0
        }
    }

    fn matches(&self) -> usize {
        self.winning.intersection(&self.numbers).count()
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
    let mut counts = vec![1usize; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let count = card.matches();
        for i in index + 1.. index + 1 + count {
            counts[i] += counts[index];
        }
    }
    println!("Part 1: {}", cards.iter().map(Card::score).sum::<i32>());
    println!("Part 2: {}", counts.iter().sum::<usize>());
}
