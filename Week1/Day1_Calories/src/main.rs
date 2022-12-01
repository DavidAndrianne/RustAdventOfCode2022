use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    println!("Searching the max calorie elf in file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let split = contents.split("\n");

    let mut elves:u32 = 0;
    let mut total:u32 = 0;
    let mut maxTotal:u32 = 0;
    let mut maxElf:u32 = 0;

    for s in split {
        if s.len() == 1 {
            elves = elves +1;
            println!("elf:{}, calories:{}", elves, total);
            if maxTotal < total {
                maxTotal = total;
                maxElf = elves;
            }
            total = 0;
        } else {
            let mut my_string:String = s.to_string();
            my_string.pop();
            let my_int = my_string.parse::<u32>()
                .unwrap();
            total = total + my_int;
            // println!("{}: {}", i, s);
        }
    }

    println!("max calories {} on elf : {}", maxTotal, maxElf);
}
