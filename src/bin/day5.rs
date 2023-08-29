use std::collections::VecDeque;
fn main() {
    // let stack = include_str!("../data/day5_stack.txt");
    let inst  = include_str!("../data/day5_instructions.txt");

    // let elves = input.split("\r\n\r\n");
    let one = vec!["H", "T", "Z", "D"];
    let two = vec!["Q", "R", "W", "T", "G", "C", "S"];
    let three = vec!["P", "B", "F", "Q", "N", "R", "C", "H"];
    let four = vec!["L", "C", "N", "F", "H", "S"];
    let five = vec!["G", "L", "F", "Q", "Z"];
    let six = vec!["V", "P", "W", "Z", "B", "R", "C", "S"];
    let seven = vec!["Z", "F", "J"];
    let eight    = vec!["D", "L", "V", "Z", "R", "H", "Q"];
    let nine = vec!["B", "H", "G", "N", "F", "Z", "L", "D"];
    let mut stack: Vec<VecDeque<&str>> = vec![VecDeque::from(one), VecDeque::from(two), VecDeque::from(three), VecDeque::from(four), VecDeque::from(five),
                                         VecDeque::from(six), VecDeque::from(seven), VecDeque::from(eight), VecDeque::from(nine)];
    let mut stack_2: Vec<VecDeque<&str>> = stack.to_owned(); 
    let inst: Vec<&str> = inst.split("\r\n").collect();

    // part1
    for item in &inst {
        let instruction: Vec<&str> = item.split_whitespace().collect();
        let how_many = instruction.get(1).unwrap().parse::<i32>().unwrap();
        let from = instruction.get(3).unwrap().parse::<i32>().unwrap();
        let to = instruction.get(5).unwrap().parse::<i32>().unwrap();
        let mut to_push: Vec<&str> = vec![];
        for _ in 1..=how_many {
            if let Some(elem) = stack.get_mut((from-1) as usize) {
                to_push.push(elem.pop_back().unwrap());
            };
            if let Some(elem) = stack.get_mut((to-1) as usize) {
                elem.push_back(to_push.pop().unwrap());
            }
        }
    } 

    

    println!("{:?}", stack);
    println!("--------------------------------------------------------------------------------------");
    // part2

    println!("{:?} \n", stack_2);
    for item in &inst {
        let instruction: Vec<&str> = item.split_whitespace().collect();
        println!("{:?}", instruction);
        let how_many = instruction.get(1).unwrap().parse::<i32>().unwrap();
        let from = instruction.get(3).unwrap().parse::<i32>().unwrap();
        let to = instruction.get(5).unwrap().parse::<i32>().unwrap();
        let mut to_push = VecDeque::new();
        println!("{:?} stack before", stack_2);
        for _ in 1..=how_many {
            if let Some(elem) = stack_2.get_mut((from-1) as usize) {
                to_push.push_front(elem.pop_back().unwrap());
            };
        }
        println!("------------------------------------------------------------------ \n");
        println!("{:?}", to_push);
        println!("------------------------------------------------------------------ \n");

        for item in &mut to_push {
            if let Some(vaje) = stack_2.get_mut((to-1) as usize) {
                vaje.push_back(item);
            } 
        }
    }
    println!("{:?} stack after", stack_2);
    for item in stack_2 {
        println!("{:?}", item.back());
    }

}