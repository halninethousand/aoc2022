fn main() {
    let input = include_str!("../data/day1.txt");

    let elves = input.split("\r\n\r\n");

    let mut elf: Vec<u32> = elves
        .map(|line| line.split("\r\n").flat_map(|num| num.parse::<u32>()).sum::<u32>())
        .collect();

    elf.sort_by(|a, b| b.cmp(a));

    println!("{:?}", elf[0]);
    // println!("{:?}", elf.iter().take(3).sum::<u32>());
}