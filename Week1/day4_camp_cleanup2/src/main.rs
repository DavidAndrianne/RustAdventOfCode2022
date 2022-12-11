use std::env;
use std::fs;
use std::vec::Vec;

use day4_camp_cleanup2::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Calculaing the total score for rucksacks in file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total: u32 = 0;
    for s in contents.split("\r\n") {
        let split = s.split(",");
        let elves: Vec<&str> = split.collect();
        
        let elf1 = elves.get(0).unwrap();
        let elf2 = elves.get(1).unwrap();
        let elf1_range = get_range(elf1);
        let elf2_range = get_range(elf2);

        let is_containing = is_range_overlapping_range(&elf1_range, &elf2_range);
        if is_containing { total = total + 1 ;}
        println!("Elf1 range: {:?}, Elf2 range: {:?}, one overlaps the other: {}", elf1_range, elf2_range, is_containing);
    }
    println!("The ranges contained in the other range count is : {}", total);
}
