fn win_lose_score(line: Vec<&str>) -> usize {
    let line: String = line.into_iter().map(|i| i.to_string()).collect::<String>();
    //dbg!(&line);
    return match line.as_str() {
        "AX" =>  4,
        "AY" =>  8,
        "AZ" =>  3,
        "BX" =>  1,
        "BY" =>  5,
        "BZ" =>  9,
        "CX" =>  7,
        "CY" =>  2,
        "CZ" =>  6,
        _ => unreachable!("nice input guy"),
    }
}
fn main() {
    let input = include_str!("../data/day2.txt");
    let lines = input.lines();
    let mut score: usize = 0;
    for line in lines {
        let line: Vec<&str> = line.split(" ").collect();
        score += win_lose_score(line);
        println!("{:?}", score);
    }
}