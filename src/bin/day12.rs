const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
        'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const alpha: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let input: Vec<Vec<char>> = include_str!("../data/day12_short.txt").lines().map(|line| line.chars().collect()).collect();
    println!("{:?}", input);
    

    let start_end_indices: Vec<(usize, usize)> = input 
        .iter()
        .enumerate()
        .flat_map(|(i, inner_vec)| {
            inner_vec
                .iter()
                .enumerate()
                .filter_map(move |(j, &c)| {
                    if c.is_uppercase() {
                        Some((i, j))
                    } else {
                        None
                    }
                })
        })
        .collect();

    let S = start_end_indices[0];
    let E = start_end_indices[1];

    let directions = possible_directions(&input, S.0, S.1);
    println!("{:?}", directions);

    println!("{:?}", start_end_indices);
}

fn possible_directions(grid: &[Vec<char>], y: usize, x: usize) -> Vec<((i32, i32), char)> {
    let offsets = [
                (-1, 0),
        (0, -1),         (0, 1),
                (1, 0),
    ];
    
    let rows = grid.len();
    let cols = grid[0].len();

    let mut directions = vec![];
    
    for (dx, dy) in offsets.iter() {
        let new_y = y as i32 + dx;
        let new_x = x as i32 + dy;
        let mut current_char = grid[y][x];

        if current_char == 'S' {
            current_char = 'a';
        }

        if new_y >= 0 && new_y < rows as i32 && new_x >= 0 && new_x < cols as i32 {
            if is_equal_or_bigger(alpha, grid[new_y as usize][new_x as usize], current_char) {
                directions.push(((new_y, new_x), grid[y][x]));
            }
            println!("new y: {} new x: {}", new_y, new_x);
        }
    }
    directions
}

fn is_equal_or_bigger(alphabet: &str, input_char: char, compare_char: char) -> bool {
    let input_index = alphabet.find(input_char);
    let compare_index = alphabet.find(compare_char);

    match (input_index, compare_index) {
        (Some(i), Some(j)) => i >= j,
        _ => false,
    }
}
