use std::collections::HashSet;
use std::str;
fn main () {
    let input = include_str!("../data/day3.txt");
    let rucksacks: Vec<&str> = input.split("\r\n").collect();
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap()
        .into_bytes();
    let scores = (1..=52).collect::<Vec<u8>>();
    let board: Vec<(u8, u8)> = alphabet.into_iter().zip(scores.into_iter()).collect();
    let mut answer: u16 = 0;

    // part1

    // for item in rucksacks {
    //     let (container1, container2) = item.split_at(item.len()/2);

    //     let container1: HashSet<u8> = Vec::from(container1).iter().cloned().collect(); 
    //     let container2: HashSet<u8> = Vec::from(container2).iter().cloned().collect(); 
    //     let intersection = container1.intersection(&container2).cloned().collect::<Vec<u8>>();

    //     let mut score = 0;

    //     for item in intersection {
    //         for letter in &board {
    //             println!("{:?}", letter);
    //             if letter.0 == item {
    //                 score += letter.1;
    //                 println!("{:?}, {:?}", char::from(letter.0), letter.1);
    //             }
    //         }
    //     }
    //     answer += score as u32;
    // }
    // println!("score for this rucksa {}", answer);

    // part2

    let by_threes: Vec<Vec<&str>> = rucksacks.chunks(3).map(|slice| slice.to_vec()).collect();
    for item in &by_threes {
        let r1: HashSet<u8> = Vec::from(*item.get(0).unwrap()).iter().cloned().collect(); 
        let r2: HashSet<u8> = Vec::from(*item.iter().nth(1).unwrap()).iter().cloned().collect(); 
        let r3: HashSet<u8> = Vec::from(*item.iter().nth(2).unwrap()).iter().cloned().collect(); 
        let one_two_int = r1.intersection(&r2).cloned().collect::<HashSet<u8>>(); 
        let two_three_int = one_two_int.intersection(&r3).cloned().collect::<Vec<u8>>(); 
        println!("{:?}, {:?}", one_two_int, two_three_int);
        for letter in &board {
            println!("{:?}", letter);
            if letter.0 == *two_three_int.get(0).unwrap() {
                answer += letter.1 as u16;
                println!("{:?}, {:?}", char::from(letter.0), letter.1);
            }
        }

    }
    
    println!("{:?}", answer);
     

}
