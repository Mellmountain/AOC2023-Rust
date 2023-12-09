#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    score: usize,
}

#[derive(Debug, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
enum Card {
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
        Self {cards: hand.to_vec(), bid, score}    
    }

    fn score(hand: &[Card]) -> usize{
        let mut sorted: Vec<Card> = hand.into();
        sorted.sort();
        
        let handtype = if sorted[0] == sorted[4] {
            HandType::FiveOfAKind
        } else if sorted[0] == sorted[3] || sorted[1] == sorted[4] {
            HandType::FourOfAKind
        } else if sorted[0] == sorted[2] && sorted[3] == sorted[4] ||
                  sorted[0] == sorted[1] && sorted[2] == sorted[4] {
            HandType::FullHouse
        } else if sorted[0] == sorted[2] || 
                  sorted[1] == sorted[3] ||
                  sorted[2] == sorted[4] {
            HandType::ThreeOfAKind
        } else if sorted[0] == sorted[1] && sorted[2] == sorted[3] ||
                  sorted[0] == sorted[1] && sorted[3] == sorted[4] ||
                  sorted[1] == sorted[2] && sorted[3] == sorted[4] {
            HandType::TwoPair
        } else if sorted[0] == sorted[1] || 
                  sorted[1] == sorted[2] ||
                  sorted[2] == sorted[3] ||
                  sorted[3] == sorted[4]{
            HandType::OnePair
        } else {
            HandType::HighCard
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
        //println!("{:?} score = {:x}", hand.cards, hand.score);
        hands.push(hand);
    }    

    hands.sort_by(|hand_1, hand_2| hand_1.score.cmp(&hand_2.score));
    let mut part1 = 0;
    for (idx, hand) in hands.iter().enumerate() {
        part1 += (idx + 1) * hand.bid;
    }
    println!("{}", part1);
}