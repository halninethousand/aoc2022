use itertools::Itertools;

fn main() {
    let input = include_str!("../data/day8.txt");

    let trees: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u32))
                .collect()
        })
        .collect();

    let size = trees.len();
    let size = size as u32; 

    let mut matrix: Vec<Vec<bool>> = vec![vec![false; size as usize]; size as usize];

    fn is_at_edge(x: u32, y: u32, size: u32) -> bool {
        let on_edge = x == 0 || x == size - 1 || y == 0 || y == size - 1;
        on_edge
    }

    fn how_many_trues(matrix: &Vec<Vec<bool>>) {
        let mut count: usize = 0;
        for item in matrix {
            for bo in item {
                if *bo {
                    count += 1;
                }
            }
        }
        eprintln!("visible = {:?}", count);
    }

    fn calculate_visibility(trees: &Vec<Vec<u32>>, size: u32, matrix: &mut Vec<Vec<bool>>) {
        for i in 0..size {
            for j in 0..size {
                let current_height = trees[i as usize][j as usize];
                if is_at_edge(i, j, size) {
                    matrix[i as usize][j as usize] = true;
                } else {
                    let right_visible = (j + 1..size).all(|y_right| trees[i as usize][y_right as usize] < current_height);
                    let left_visible = (0..j).rev().all(|y_left| trees[i as usize][y_left as usize] < current_height);
                    let down_visible = (i + 1..size).all(|x_down| trees[x_down as usize][j as usize] < current_height);
                    let up_visible = (0..i).rev().all(|x_up| trees[x_up as usize][j as usize] < current_height);

                    let result = right_visible || left_visible || down_visible || up_visible;

                    if result {
                        matrix[i as usize][j as usize] = true;
                    }
                }
            }
        }
    }

    fn calculate_scenic_score(trees: &Vec<Vec<u32>>, x: u32, y: u32, size: u32, biggest: &mut u32) -> u32 {

        let current_height = trees[x as usize][y as usize];

        let mut up_score = (0..x).rev().take_while_inclusive(|&i| trees[i as usize][y as usize] < current_height).count() as u32;
        let mut down_score = (x + 1..size).take_while_inclusive(|&i| trees[i as usize][y as usize] < current_height).count() as u32;
        let mut left_score = (0..y).rev().take_while_inclusive(|&j| trees[x as usize][j as usize] < current_height).count() as u32;
        let mut right_score = (y + 1..size).take_while_inclusive(|&j| trees[x as usize][j as usize] < current_height).count() as u32;

        if up_score == 0 {
            up_score = 1;
        } 
        if down_score == 0 {
            down_score = 1;
        } 
        if left_score == 0 {
            left_score = 1;
        } 
        if right_score == 0 {
            right_score = 1;
        } 

        let score = up_score * down_score * left_score * right_score;

        if score > *biggest {
            *biggest = score;
        }
    
        score
    }

    let mut biggest:u32 = 0;

    for i in 0..size {
        for j in 0..size {
            calculate_scenic_score(&trees, i, j, size, &mut biggest);
        }
    }

    //println!("matrix {:?}", matrix);
    calculate_visibility(&trees, size, &mut matrix);
    //println!("matrix {:?}", matrix);
    how_many_trues(&matrix);
    println!("The highest scenic score is: {}", biggest);
}
