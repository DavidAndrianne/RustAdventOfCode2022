use std::env;
use std::fs;

use Day3_RucksackInventorization2::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Calculaing the total score for rucksacks in file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total:u32 = 0;
    let mut str1: String = "txt1".to_string();
    let mut str2: String = "txt2".to_string();
    let mut str3: String = "txt3".to_string();
    let mut i: u32 = 0;
    for s in contents.split("\r\n") {
        str3 = str2;
        str2 = str1;
        str1 = s.to_string();

        i = i +1;

        if i % 3 != 0 { continue; }

        let common_characters = identify_common_characters(&str1, &str2);
        let common_characters2 = identify_common_characters(&str1, &str3);

        let common_string: String = common_characters.into_iter().collect();
        let common_string2: String = common_characters2.into_iter().collect();

        let mut common_characters_all_3_strings = identify_common_characters(&common_string, &common_string2);
        common_characters_all_3_strings = filter_uniq(common_characters_all_3_strings);

        if common_characters_all_3_strings.len() != 1 { 
            println!("ERROR: found{:?} in {}| |{}| |{}!", common_characters_all_3_strings, &str1, &str2, &str3 );
            continue;
        }

        let c = common_characters_all_3_strings.first().unwrap();

        let byte_score = get_priority(*c);
        // let score_as_int = [byte_score].map(Into::<u32>::into);
        // let score_as_option_int = score_as_int.first();
        let int_score = byte_score as u32;
        println!("{}:{}, {}| |{}| |{}", c, int_score, str1, str2, str3);
        total = total + int_score; //score_as_option_int.unwrap(); // 7431 -> too high
    }

    println!("total score {total}");
}