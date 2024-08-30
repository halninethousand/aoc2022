use std::collections::{VecDeque, HashSet};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let input: Vec<Vec<char>> = include_str!("../data/day12.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_end_indices: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, inner_vec)| {
            inner_vec
                .iter()
                .enumerate()
                .filter_map(move |(j, &c)| {
                    if c == 'S' || c == 'E' {
                        Some((i, j))
                    } else {
                        None
                    }
                })
        })
        .collect();

    let start = start_end_indices[0];
    let end = start_end_indices[1];
    
    if let Some(steps) = bfs(&input, start, end) {
        println!("Found a path with {} steps", steps);
    } else {
        println!("No valid path found");
    }

    let start_points: Vec<(usize, usize)> = input.iter()
        .enumerate()
        .flat_map(|(i, inner_vec)| {
            inner_vec.iter()
                .enumerate()
                .filter_map(move |(j, &c)| {
                    if c == 'a' || c == 'S' {  // Treat 'S' as 'a'
                        Some((i, j))
                    } else {
                        None
                    }
                })
        })
        .collect();

    // Locate the end position 'E'
    let end = input.iter()
        .enumerate()
        .flat_map(|(i, inner_vec)| {
            inner_vec.iter()
                .enumerate()
                .filter_map(move |(j, &c)| {
                    if c == 'E' {
                        Some((i, j))
                    } else {
                        None
                    }
                })
        })
        .next()
        .expect("End position 'E' not found!");

    let mut min_steps: Option<usize> = None;

    for start in start_points {
        if let Some(steps) = bfs(&input, start, end) {
            min_steps = match min_steps {
                Some(current_min) => Some(current_min.min(steps)),
                None => Some(steps),
            };
        }
    }

    match min_steps {
        Some(steps) => println!("The shortest path takes {} steps.", steps),
        None => println!("No valid path found from any 'a' position."),
    }
}

fn bfs(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some(((y, x), steps)) = queue.pop_front() {
        if (y, x) == end {
            return Some(steps);
        }

        let directions = possible_directions(grid, y, x);
        for ((new_y, new_x), _) in directions {
            let new_pos = (new_y as usize, new_x as usize);
            if !visited.contains(&new_pos) {
                visited.insert(new_pos);
                queue.push_back((new_pos, steps + 1));
            }
        }
    }

    None
}

fn possible_directions(grid: &[Vec<char>], y: usize, x: usize) -> Vec<((i32, i32), char)> {
    let offsets = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];
    
    let rows = grid.len();
    let cols = grid[0].len();
    let mut directions = vec![];
    let current_char = match grid[y][x] {
        'S' => 'a',
        'E' => 'z',
        other => other,
    };

    for &(dy, dx) in &offsets {
        let new_y = y as i32 + dy;
        let new_x = x as i32 + dx;

        if new_y >= 0 && new_y < rows as i32 && new_x >= 0 && new_x < cols as i32 {
            let adjacent = match grid[new_y as usize][new_x as usize] {
                'S' => 'a',
                'E' => 'z',
                other => other,
            };
            if is_equal_or_one_bigger(current_char, adjacent) {
                directions.push(((new_y, new_x), adjacent));
            }
        }
    }
    directions
}

fn is_equal_or_one_bigger(current: char, adjacent: char) -> bool {
    let current_index = ALPHABET.find(current);
    let adjacent_index = ALPHABET.find(adjacent);

    match (current_index, adjacent_index) {
        (Some(c), Some(a)) => a <= c + 1,
        _ => false,
    }
}
