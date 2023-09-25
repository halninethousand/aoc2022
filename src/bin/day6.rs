use std::hash::Hash;
use std::collections::HashSet;

fn main() {
    println!("asd");
    let letters: Vec<_> = include_str!("../data/day6.txt").chars().collect();

    // part1
    for i in 0..letters.len() {
        let slc = letters.iter().skip(i).take(4).collect::<Vec<&char>>();
        println!("{:?}", slc);
        match has_unique_elements(slc) {
            true => {
                println!("{} index has no dupl", i+4);
                break;
            },
            false => (),
        }
    }

    // part2
    for i in 0..letters.len() {
        let slc = letters.iter().skip(i).take(14).collect::<Vec<&char>>();
        println!("{:?}", slc);
        match has_unique_elements(slc) {
            true => {
                println!("{} index has no dupl", i+14);
                break;
            },
            false => (),
        }
    }
} 

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}