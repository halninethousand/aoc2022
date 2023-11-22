fn main() {
    let input = include_str!("../data/day8.txt");

    let trees: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                .collect()
        })
        .collect();

    let size = trees.len();

    let mut matrix: Vec<Vec<bool>> = vec![vec![false; size]; size];

    fn is_at_edge(x: usize, y: usize, size: usize) -> bool {
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
        eprintln!("count = {:?}", count);
    }
    
    fn calculate_visibility(trees: &Vec<Vec<usize>>, size: usize, matrix: &mut Vec<Vec<bool>>) -> () {
        for i in 0..size {
            for j in 0..size {
                let current_height = trees[i][j];
                if is_at_edge(i, j, size) {
                    println!("{} {} at edge", i, j);
                    matrix[i][j] = true;
                } else {
                    let right_visible = (j + 1..size).all(|y_right| trees[i][y_right] < current_height);

                    let left_visible = (0..j).rev().all(|y_left| trees[i][y_left] < current_height);
            
                    let down_visible = (i + 1..size).all(|x_down| trees[x_down][j] < current_height);
            
                    let up_visible = (0..i).rev().all(|x_up| trees[x_up][j] < current_height);
                    
                    let result = right_visible || left_visible || down_visible || up_visible;
                    
                    if result {
                        matrix[i][j] = true;
                    } 

                    println!("number {} is {} ||| {} {} {} {}", trees[i][j], result, right_visible, left_visible, down_visible, up_visible);
                }

            }
        }
    }
    println!("matrix {:?}", matrix); 
    calculate_visibility(&trees, size, &mut matrix);
    println!("matrix {:?}", matrix); 
    how_many_trues(&matrix);
}
