use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let filename = std::env::args().nth(1).unwrap();
    let content = read_to_string(filename)?;
    let as_int: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| -> i32 {
                    match ch {
                        '.' => -1i32,
                        _ => 0i32,
                    }
                })
                .collect()
        })
        .collect();

    println!("{}", count_removed(as_int));

    Ok(())
}

fn count_removed(mut matrix: Vec<Vec<i32>>) -> i32 {
    let mut res = count_empty(&mut matrix);
    let mut modified: bool = remove_possible_rolls(&mut matrix);
    while modified == true {
        res += count_empty(&mut matrix);
        modified = remove_possible_rolls(&mut matrix);
    }
    res
}

fn count_empty(matrix: &mut Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut res = 0;
    for row in 0..n {
        for col in 0..m {
            if matrix[row][col] != -1 {
                if col > 0 {
                    matrix[row][col] += is_adjacent_roll(matrix[row][col - 1]);
                    if row > 0 {
                        matrix[row][col] += is_adjacent_roll(matrix[row - 1][col - 1]);
                    }

                    if row < n - 1 {
                        matrix[row][col] += is_adjacent_roll(matrix[row + 1][col - 1]);
                    }
                }

                if col < m - 1 {
                    matrix[row][col] += is_adjacent_roll(matrix[row][col + 1]);
                    if row > 0 {
                        matrix[row][col] += is_adjacent_roll(matrix[row - 1][col + 1]);
                    }

                    if row < n - 1 {
                        matrix[row][col] += is_adjacent_roll(matrix[row + 1][col + 1]);
                    }
                }

                if row > 0 {
                    matrix[row][col] += is_adjacent_roll(matrix[row - 1][col]);
                }

                if row < n - 1 {
                    matrix[row][col] += is_adjacent_roll(matrix[row + 1][col]);
                }

                if matrix[row][col] < 4 {
                    res += 1;
                    matrix[row][col] = 9;
                } else {
                    matrix[row][col] = 0;
                }
            }
        }
    }

    res
}

fn remove_possible_rolls(matrix: &mut Vec<Vec<i32>>) -> bool {
    let mut modified = false;
    let n = matrix.len();
    let m = matrix[0].len();
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 9 {
                matrix[i][j] = -1;
                modified = true;
            }
        }
    }
    modified
}

fn is_adjacent_roll(val: i32) -> i32 {
    if val >= 0 {
        return 1i32;
    }
    0i32
}
