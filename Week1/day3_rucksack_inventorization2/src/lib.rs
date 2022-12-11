use std::vec::Vec;
use std::collections::HashSet;

pub fn identify_common_characters(compartment1:&str, compartment2:&str) -> Vec<char>{
    let mut commonCharacters: Vec<char> = Vec::<char>::new(); 
    for (_i, c1) in compartment1.chars().enumerate() {
        for (_i2, c2) in compartment2.chars().enumerate() {
            if c1 == c2 { commonCharacters.push(c1); }
        }
    }

    return commonCharacters;
}

// stolen from https://github.com/Jomy10/Advent-Of-Code-2022/blob/master/day3/src/lib.rs
pub fn get_priority(item: char) -> u8 {
    if item >= 'a' {
        (item as u8) - ('a' as u8) + 1
    } else {
        (item as u8) - ('A' as u8) + 27
    }
}

pub fn filter_uniq(vec: Vec<char>) -> Vec<char> {
    vec.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}