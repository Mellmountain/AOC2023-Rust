#[derive(Debug)]
struct Hand {
    _cards: Vec<Card>,
    bid: usize,
    score: usize,
    wildcard: usize,
}

#[derive(Debug, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn is(self, other:Card) -> bool {
        if self == Self::Joker || other == Self::Joker {
            false
        } else {
            self == other
        }
    }
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _   => panic!("input is not correct!"),
        }
    }
}

impl Hand {
    fn new (hand: &[Card], bid: usize) -> Self {
        let score = Self::score(hand);
        let wildcard_score = Self::score_wildcard(hand);
        Self {_cards: hand.to_vec(), bid, score, wildcard: wildcard_score}    
    }

    fn score_wildcard(hand : &[Card]) -> usize {
        let wildcard_hand: Vec<Card> = hand.iter().map(|card| {
           if *card == Card::Jack {
            Card::Joker
           } else {
            *card
           }
        }).collect();
        Self::score(&wildcard_hand)
        
    }
    
    fn score(hand: &[Card]) -> usize{
        let mut sorted: Vec<Card> = hand.into();
        sorted.sort();
        
        let joker_count = hand.iter().filter(|card| **card == Card::Joker).count();
        let handtype = if sorted[0] == sorted[4] {
            HandType::FiveOfAKind
        } else if sorted[0].is(sorted[3]) || sorted[1].is(sorted[4]) {
            if joker_count == 1 {
                HandType::FiveOfAKind
            } else {
                HandType::FourOfAKind
            }
        } else if sorted[0].is(sorted[2]) && sorted[3].is(sorted[4]) ||
                  sorted[0].is(sorted[1]) && sorted[2].is(sorted[4]) {
            HandType::FullHouse
        } else if sorted[0].is(sorted[2]) || 
                  sorted[1].is(sorted[3]) ||
                  sorted[2].is(sorted[4]) {
            if joker_count == 0 {
                HandType::ThreeOfAKind
            } else if joker_count == 1 {
                HandType::FourOfAKind
            } else {
                HandType::FiveOfAKind
            }
            
        } else if sorted[0].is(sorted[1]) && sorted[2].is(sorted[3])||
                  sorted[0].is(sorted[1]) && sorted[3].is(sorted[4])||
                  sorted[1].is(sorted[2]) && sorted[3].is(sorted[4]) {
            if joker_count == 1 {
                HandType::FullHouse
            } else {
                HandType::TwoPair
            }
        } else if sorted[0].is(sorted[1]) || 
                  sorted[1].is(sorted[2]) ||
                  sorted[2].is(sorted[3]) ||
                  sorted[3].is(sorted[4]){
            
            if joker_count == 3 {
                HandType::FiveOfAKind
            } else if joker_count == 2 {
                HandType::FourOfAKind
            } else if joker_count == 1 {
                HandType::ThreeOfAKind
            } else {
                HandType::OnePair
            }
        } else {
            if joker_count == 4 {
                HandType::FiveOfAKind
            } else if joker_count == 3 {
                HandType::FourOfAKind
            } else if joker_count == 2 {
                HandType::ThreeOfAKind
            } else if joker_count == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        };
        
        let mut score: usize = handtype as usize;
        for card in hand {
            score = (score << 4) | (*card as usize)
        }
        score
    }
}

fn main() {
    let input = include_str!("./input.txt").lines();
    let mut hands:Vec<Hand> = Vec::new();
    for line in input{
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse().unwrap();

        let hand = hand.chars().map(|c| Card::from_char(c)).collect::<Vec<_>>();
        let hand = Hand::new(&hand, bid);
        hands.push(hand);
    }    

    hands.sort_by(|hand_1, hand_2| hand_1.score.cmp(&hand_2.score));
    let mut part1 = 0;
    for (idx, hand) in hands.iter().enumerate() {
        part1 += (idx + 1) * hand.bid;
    }
    hands.sort_by(|hand_1, hand_2| hand_1.wildcard.cmp(&hand_2.wildcard));
    let mut part2 = 0;
    for(idx, hand) in hands.iter().enumerate() {
        part2 += (idx + 1) * hand.bid
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}