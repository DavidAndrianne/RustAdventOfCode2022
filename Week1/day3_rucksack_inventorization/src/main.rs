use std::env;
use std::fs;
// use std::collections::HashMap;

use Day3_RucksackInventorization::*;

fn main() {
    // let alphabeth:&str = "abcdefghijklmnopqrstuvwxyz";

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Calculaing the total score for rucksacks in file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // let scoreDictLower = init_alphabeth_dict(alphabeth, false);
    // let scoreDictUpper = init_alphabeth_dict(alphabeth, true);
    // for (key, score) in &scoreDictLower {
    //     println!("{key} results in {score} score");
    // }

    let mut total:u32 = 0;
    for s in contents.split("\r\n") {
        let mut my_string:String = s.to_string();

        let (compartment1, compartment2) = my_string.split_at(my_string.len()/2);
        let c = identify_common_character(compartment1, compartment2);
        let byte_score = get_priority(c);
        // let score_as_int = [byte_score].map(Into::<u32>::into);
        // let score_as_option_int = score_as_int.first();
        let int_score = byte_score as u32;
        println!("{}| |{}, {}:{}", compartment1, compartment2, c, int_score);
        total = total + int_score; //score_as_option_int.unwrap(); // 7431 -> too high
    }

    println!("total score {total}");
}