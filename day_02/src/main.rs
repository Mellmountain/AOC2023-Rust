use std::collections::HashMap;

struct Game {
    red:   i32,
    blue:  i32,
    green: i32,
}
impl Game {
    fn new(red: i32, blue: i32, green: i32) -> Self {
        Game {red,  blue, green}
    }
    
}

fn main() {
    let mut input = include_str!("./input.txt").lines();
    let mut games: HashMap<i32, Game> = HashMap::new();    
}
