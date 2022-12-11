pub fn identify_common_character(compartment1:&str, compartment2:&str) -> char{
    for (_i, c1) in compartment1.chars().enumerate() {
        for (_i2, c2) in compartment2.chars().enumerate() {
            if c1 == c2 { return c1; }
        }
    }

    return '?';
}

// stolen from https://github.com/Jomy10/Advent-Of-Code-2022/blob/master/day3/src/lib.rs
pub fn get_priority(item: char) -> u8 {
    if item >= 'a' {
        (item as u8) - ('a' as u8) + 1
    } else {
        (item as u8) - ('A' as u8) + 27
    }
}

/*
fn init_alphabeth_dict<'a>(alphabeth:&'a str, toUpper:bool) -> HashMap<&'a str, u32> {
    println!{"{}", alphabeth};
    let mut scoreDict:HashMap<&'a str, u32> = HashMap::new();

    let mut index:u32 = 0;
    if toUpper {
        index = index + 26;
    }
    for (_i, c) in alphabeth.chars().enumerate() {
        index = index +1;
        let key: &'a str;
        if toUpper {
            let charUpperCased: &'a _ = &c.to_ascii_uppercase();
            // let charUpperString: String = charUpperCased.collect(); 
            let charString: &'a String = charUpperCased.to_string();
            key = charString.as_str();
        } else {
            let charString = &c.to_string();
            key = charString;
        }
        scoreDict.insert(key, index);
    }

    scoreDict
}
*/