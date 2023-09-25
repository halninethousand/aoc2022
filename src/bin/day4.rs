fn main () {
    let input = include_str!("../data/day4.txt");
    // let elves: Vec<&str> = input.split(",");
    let elves: Vec<&str> = input.split("\r\n").collect();
    let elves: Vec<_> = elves.into_iter().map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<_>>();
    let mut score = 0;
    let mut score_2 = 0;
    for item in &elves {
        println!("{:?}", item);
        let (left_low, left_high) = item.get(0).unwrap().split_once("-").unwrap();
        let (right_low, right_high) = item.get(1).unwrap().split_once("-").unwrap();
        println!("{:?}", left_low.parse::<i32>().unwrap());
        println!("{:?}, {:?}, {:?}, {:?}", left_low, left_high, right_low, right_high);
        if (left_low.parse::<i32>().unwrap() <= right_low.parse::<i32>().unwrap() 
            && right_low.parse::<i32>().unwrap() <= left_high.parse::<i32>().unwrap() 
            && right_high.parse::<i32>().unwrap() <= left_high.parse::<i32>().unwrap())
                || (right_low.parse::<i32>().unwrap() <= left_low.parse::<i32>().unwrap() 
                && left_low.parse::<i32>().unwrap() <= right_high.parse::<i32>().unwrap() 
                && left_high.parse::<i32>().unwrap() <= right_high.parse::<i32>().unwrap()) {
                    score += 1; 
                    println!("adding")
            }
        println!("{:?}", score);
        if (left_low.parse::<i32>().unwrap() <= right_low.parse::<i32>().unwrap() 
            && right_low.parse::<i32>().unwrap() <= left_high.parse::<i32>().unwrap()) 
                || (right_low.parse::<i32>().unwrap() <= left_low.parse::<i32>().unwrap() 
                && left_low.parse::<i32>().unwrap() <= right_high.parse::<i32>().unwrap()) { 
                    score_2 += 1; 
                    println!("adding")
            }
    } 
    println!("{:?}", score);
    println!("{:?}", score_2);
}