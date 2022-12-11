use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Calculaing the total score for strategy in file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let scoreDict = init_dict();
    // print dict
    for (key, score) in &scoreDict {
        println!("{key} results in {score} score");
    }

    let mut total:u32 = 0;
    for s in contents.split("\n") {
        let mut my_string:String = s.to_string();
        if my_string.len() > 3 { 
            my_string.pop();
        };

        let lineArgs:Vec<&str> = my_string.split(' ').collect();
        let opponent = lineArgs.get(0).unwrap();
        let me = lineArgs.get(1).unwrap();
        let (key, val) = scoreDict.get_key_value(&my_string as &str).unwrap();
        println!("{:?} - {:?}",key, val);
        // println!("opponent plays {}, I should play {} for a score of {}", opponent, me, &score);
        total = total + val;
    }

    println!("total score {total}");
}

fn init_dict() -> HashMap<&'static str, u32> {
    let mut scoreDict:HashMap<&str, u32> = HashMap::from([
    // X : loss
    // Y : Tie
    // Z : Win

    ("A X", 3), // Rock & Scissors (3) -> Loss (0)
    ("A Y", 4), // Rock & Rock (1) -> Tie (3)
    ("A Z", 8), // Rock & Paper (2) -> Win (6)
    ("B X", 1), // Paper & Rock (1) -> Loss (0)
    ("B Y", 5), // Paper & Paper (2) -> Tie (3)
    ("B Z", 9), // Paper & Scissors (3) -> Win (6)
    ("C X", 2), // Scissors & Paper (2) -> Loss (0)
    ("C Y", 6), // Scissors & Scissors (3) -> Tie (3)
    ("C Z", 7), // Scissors & Rock (1) -> Win (6)
    ]);
    scoreDict
}

fn score(opponent:&str, me:&str){

}