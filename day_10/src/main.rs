use std::{collections::HashMap, cell};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cell {
    position: (i32, i32),
    celltype: CellType,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum CellType {
    Empty,          // "."
    Start,          // "S"
    NorthSouth,     // "|"
    EastWest,       // "-"
    NorthEast,      // "L"
    NorthWest,      // "J"
    SouthWest,      // "7"
    SouthEast,      // "F"
}

impl CellType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => CellType::Empty,
            'S' => CellType::Start,
            '|' => CellType::NorthSouth,
            '-' => CellType::EastWest,
            'L' => CellType::NorthEast,
            'J' => CellType::NorthWest,
            '7' => CellType::SouthWest,
            'F' => CellType::SouthEast,
              _ => panic!("Nooooooooooo! Input is wrong I tell you!"),     
        }

    }
}

fn main() {
    let input = include_str!("./input.txt").lines();
    let mut map: Vec<Vec<CellType>> = Vec::new();
    let mut start:(i32, i32) = (0, 0);
    let mut pipe_loop: Vec<Cell> = Vec::new();
    for (y, line) in input.into_iter().enumerate() {
        let cells = line.chars().map(|c| CellType::from_char(c)).collect::<Vec<_>>();
        for (x, c) in cells.iter().enumerate() {
            if *c == CellType::Start {
                start = (x as i32, y as i32);
                pipe_loop.push(Cell {position: start, celltype: *c});
            }
        }
        map.push(cells);
    }
    let mut pos = start;
    let mut visited: HashMap<(i32, i32), CellType> = HashMap::new();
    visited.insert(start, CellType::Start);
    let mut celltype = map[pos.1 as usize][pos.0 as usize];
    loop {
        //println!("{:?}", celltype);
        //println!("{:?}", pos);
        match celltype {
            CellType::Start => {
                if can_go_north(pos, map.clone(), visited.clone()) { pos = (pos.0, pos.1 - 1);}
                else if can_go_south(pos, map.clone(), visited.clone()) { pos = (pos.0, pos.1 + 1);}
                else if can_go_east(pos, map.clone(), visited.clone())  { pos = (pos.0 + 1, pos.1);}
                else if can_go_west(pos, map.clone(), visited.clone())  { pos = (pos.0 - 1, pos.1);}
                else {
                    if can_go_start(pos, start) { pos = start; }
                    else {
                        panic!("code bug, shoot me")
                    }
                }
            }
            CellType::EastWest => {
                if can_go_east(pos, map.clone(), visited.clone()) { pos = (pos.0 + 1, pos.1);}
                else if can_go_west(pos, map.clone(), visited.clone()) { pos = (pos.0 - 1, pos.1);}
                else {
                    if can_go_start(pos, start) { pos = start; }
                    else {
                        panic!("code bug, shoot me")
                    }
                }
            }
            CellType::NorthSouth => {
                if can_go_north(pos, map.clone(), visited.clone()) { pos = (pos.0, pos.1 - 1);}
                else if can_go_south(pos, map.clone(), visited.clone()) { pos = (pos.0, pos.1 + 1);}
                else {
                    if can_go_start(pos, start) { pos = start; }
                    else {
                        panic!("code bug")
                    }
                }
            }
            CellType::NorthEast => {
                if can_go_north(pos, map.clone(), visited.clone()) { pos = (pos.0, pos.1 - 1);}
                else if can_go_east(pos, map.clone(), visited.clone())  { pos = (pos.0 + 1, pos.1);}
                else {
                    if can_go_start(pos, start) { pos = start; }
                    else {
                        panic!("code bug")
                    }
                }
            }
            CellType::NorthWest => {
                if can_go_north(pos, map.clone(), visited.clone()) { pos = (pos.0, pos.1 - 1);}
                else if can_go_west(pos, map.clone(), visited.clone())  { pos = (pos.0 - 1, pos.1);}
                else {
                    if can_go_start(pos, start) { pos = start; }
                    else {
                        panic!("code bug")
                    }
                }
            }
            CellType::SouthWest => {
                if can_go_south(pos, map.clone(), visited.clone()) { pos = (pos.0, pos.1 + 1);}
                else if can_go_west(pos, map.clone(), visited.clone())  { pos = (pos.0 - 1, pos.1);}
                else {
                    if can_go_start(pos, start) { pos = start; }
                    else {
                        panic!("code bug")
                    }
                }
            }
            CellType::SouthEast => {
                if can_go_south(pos, map.clone(), visited.clone()) { pos = (pos.0, pos.1 + 1);}
                else if can_go_east(pos, map.clone(), visited.clone())  { pos = (pos.0 + 1, pos.1);}
                else {
                    if can_go_start(pos, start) { pos = start; }
                    else {
                        panic!("code bug")
                    }
                }
            }
            _ => { panic!("Code bug deluxe!")}
        }
        
        //println!("{:?}", pipe_loop);
        celltype = map[pos.1 as usize][pos.0 as usize];
        if celltype == CellType::Start { break }
        visited.insert(pos, celltype);
        pipe_loop.push(Cell {position: pos, celltype: celltype});

    }
    //println!("{:?}", start);
    //println!("{:?}", map[start.1 as usize][start.0 as usize]);
    println!("{}", pipe_loop.len() / 2);
}
fn can_go_start(pos: (i32, i32), start: (i32, i32)) -> bool {
    if (pos.0 - 1, pos.1) == start ||
       (pos.0 + 1, pos.1) == start ||
       (pos.0, pos.1 + 1) == start ||
       (pos.0, pos.1 - 1) == start {
        return true
       }
    false
}

fn can_go_north(pos: (i32, i32), map: Vec<Vec<CellType>>, visited: HashMap<(i32,i32), CellType>) -> bool {
    let newpos = (pos.0, pos.1 - 1);
    if newpos.1 > map.len().try_into().unwrap() || visited.contains_key(&newpos) { return false }
    let possible = map[newpos.1 as usize][newpos.0 as usize]; 
    if possible == CellType::NorthSouth ||
       possible == CellType::SouthWest  ||
       possible == CellType::SouthEast {
        return true
    }
    false
}

fn can_go_south(pos: (i32, i32), map: Vec<Vec<CellType>>, visited: HashMap<(i32,i32), CellType>) -> bool {
    let newpos = (pos.0, pos.1 + 1);
    if newpos.1 < 0 || visited.contains_key(&newpos){ return false }
    let possible = map[newpos.1 as usize][newpos.0 as usize];
    if possible == CellType::NorthSouth ||
       possible == CellType::NorthEast  ||
       possible == CellType::NorthWest {
        return true
    }
    false
}

fn can_go_east(pos: (i32, i32), map: Vec<Vec<CellType>>, visited: HashMap<(i32,i32), CellType>) -> bool {
    let newpos = (pos.0 + 1, pos.1);
    if newpos.0 > map[0].len().try_into().unwrap() || visited.contains_key(&newpos) { return false }
    let possible = map[newpos.1 as usize][newpos.0 as usize];
    if possible == CellType::EastWest  ||
       possible == CellType::SouthWest ||
       possible == CellType::NorthWest {
        return true
    }
    false
}

fn can_go_west(pos: (i32, i32), map: Vec<Vec<CellType>>, visited: HashMap<(i32,i32), CellType>) -> bool {
    let newpos = (pos.0 - 1, pos.1);
    if newpos.0 < 0 || visited.contains_key(&newpos) { return false }
    let possible = map[newpos.1 as usize][newpos.0 as usize];
    if possible == CellType::EastWest  ||
       possible == CellType::NorthEast ||
       possible == CellType::SouthEast {
        return true
    }
    false
}
