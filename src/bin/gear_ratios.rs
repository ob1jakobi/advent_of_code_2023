use std::{env, fs};
use std::path::PathBuf;

/// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you
/// up to the *water source*, but this is as far as he can bring you. You go inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, row_num
/// wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before
/// row_num can fix it." You offer to help.
///
/// The engineer explains that an engine part seems to be missing from the engine, but nobody can
/// figure out which one. If you can *add up all the part numbers* in the engine schematic, it should
/// be easy to work out which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation of the engine.
/// There are lots of numbers and symbols you don't really understand, but apparently *any number
/// adjacent to a symbol*, even diagonally, is a "part number" and should be included in your sum.
/// (Periods (`.`) do not count as a symbol.)
///
/// Here is an example engine schematic:
/// ```
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// ```
///
/// In this schematic, two numbers are *not* part numbers because they are not adjacent to a symbol:
/// `114` (top right) and `58` (middle right). Every other number is adjacent to a symbol and so is a
/// part number; their sum is `4361`.
///
/// Of course, the actual engine schematic is much larger. *What is the sum of all of the part
/// numbers in the engine schematic?*
fn part_1(input: &str) -> usize {
    // 537732
    let is_symbol = |c: char| !c.is_digit(10) && c != '.' && c != '\n';
    let mut s_nums: Vec<String> = Vec::new();

    for (row_num, line) in input.lines().enumerate() {
        let mut is_valid_num: bool = false;
        let mut s = String::new();

        for (col_num, ch) in line.char_indices() {
            if ch.is_digit(10) {
                let (left, right) = match (row_num, col_num) {
                    (x, y) if y == 0 => ((x, y), (x, y + 1)),
                    (x, y) => ((x, y - 1), (x, y + 1)),
                };
                let (up, down) = match (row_num, col_num) {
                    (x, y) if x == 0 => ((x, y), (x + 1, y)),
                    (x, y) => ((x - 1, y), (x + 1, y)),
                };
                let (diag_ul, diag_ur) = match (row_num, col_num) {
                    (x, y) if x == 0 && y == 0 => ((x, y), (x, y + 1)),
                    (x, y) if x == 0 => ((x, y - 1), (x, y + 1)),
                    (x, y) if y == 0 => ((x - 1, y), (x - 1, y + 1)),
                    (x, y) => ((x - 1, y - 1), (x - 1, y + 1)),
                };
                let (diag_ll, diag_lr) = match (row_num, col_num) {
                    (x, y) if y == 0 => ((x + 1, y), (x + 1, y + 1)),
                    (x, y) => ((x + 1, y - 1), (x + 1, y + 1)),
                };

                let neighbors: [(usize, usize); 8] = [
                    left, right,
                    up, down,
                    diag_ul, diag_ur,
                    diag_ll, diag_lr,
                ];

                is_valid_num = neighbors
                    .iter()
                    .filter_map(|&(x, y)| {
                        input.lines().nth(x).and_then(|l| l.chars().nth(y))
                    })
                    .fold(is_valid_num, |acc, c| acc || is_symbol(c));

                s.push(ch);
            } else {
                if is_valid_num {
                    s_nums.push(s.clone());
                }
                is_valid_num = false;
                s.clear();
            }
        }
    }
    s_nums.into_iter().filter_map(|s| s.parse::<usize>().ok()).sum()

    /*
    //let num_lines = input.lines().collect::<Vec<&str>>().len();
    let two_d_chars: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>()).collect();

    let is_symbol = |c: char| !c.is_digit(10) && c != '.' && c != '\n';
    //let mut nums: Vec<usize> = Vec::new();
    let mut is_valid_num = false;

    let temp = two_d_chars
        .iter()
        .enumerate()
        .map(|(row_num, line)| {
            line
                .iter()
                .enumerate()
                .filter_map(|(col_num, ch)| {
                    if ch.is_digit(10) {
                        let (left, right) = match (row_num, col_num) {
                            (x, y) if y == 0 => ((x, y), (x, y + 1)),
                            (x, y) => ((x, y - 1), (x, y + 1)),
                        };
                        let (up, down) = match (row_num, col_num) {
                            (x, y) if x == 0 => ((x, y), (x + 1, y)),
                            (x, y) => ((x - 1, y), (x + 1, y)),
                        };
                        let (diag_ul, diag_ur) = match (row_num, col_num) {
                            (x, y) if x == 0 && y == 0 => ((x, y), (x, y + 1)),
                            (x, y) if x == 0 => ((x, y - 1), (x, y + 1)),
                            (x, y) if y == 0 => ((x - 1, y), (x - 1, y + 1)),
                            (x, y) => ((x - 1, y - 1), (x - 1, y + 1)),
                        };
                        let (diag_ll, diag_lr) = match (row_num, col_num) {
                            (x, y) if y == 0 => ((x + 1, y), (x + 1, y + 1)),
                            (x, y) => ((x + 1, y - 1), (x + 1, y + 1)),
                        };

                        let neighbors: [(usize, usize); 8] = [
                            left, right,
                            up, down,
                            diag_ul, diag_ur,
                            diag_ll, diag_lr,
                        ];

                        is_valid_num = neighbors.iter().filter_map(|&(x, y)| {
                            two_d_chars.iter().nth(x).and_then(|l| l.iter().nth(y))
                        })
                            .fold(is_valid_num, |acc, c| acc || is_symbol(*c));

                        if is_valid_num {
                            Some(ch)
                        } else {
                            None
                        }
                    } else {
                        is_valid_num = false;
                        None
                    }
                })
                .collect::<String>()
        })
        .filter_map(|s| s.parse::<usize>().ok());
    temp.sum()
    */
    /*
    for (row_num, line) in input.lines().enumerate() {
        let mut s_num: String = String::new();
        let mut is_valid_num: bool = false;
        for (col_num, ch) in line.char_indices() {
            if ch.is_digit(10) {
                let (left, right) = match (row_num, col_num) {
                    (x, y) if y == 0 => ((x, y), (x, y + 1)),
                    (x, y) => ((x, y - 1), (x, y + 1)),
                };
                let (up, down) = match (row_num, col_num) {
                    (x, y) if x == 0 => ((x, y), (x + 1, y)),
                    (x, y) => ((x - 1, y), (x + 1, y)),
                };
                let (diag_ul, diag_ur) = match (row_num, col_num) {
                    (x, y) if x == 0 && y == 0 => ((x, y), (x, y + 1)),
                    (x, y) if x == 0 => ((x, y - 1), (x, y + 1)),
                    (x, y) if y == 0 => ((x - 1, y), (x - 1, y + 1)),
                    (x, y) => ((x - 1, y - 1), (x - 1, y + 1)),
                };
                let (diag_ll, diag_lr) = match (row_num, col_num) {
                    (x, y) if y == 0 => ((x + 1, y), (x + 1, y + 1)),
                    (x, y) => ((x + 1, y - 1), (x + 1, y + 1)),
                };
                let neighbors: [(usize, usize); 8] = [
                    // Horizontally adjacent
                    left, right,
                    // Vertically adjacent
                    up, down,
                    // Diagonally adjacent
                    diag_ul, diag_ur,
                    diag_ll, diag_lr,
                ];
                neighbors.iter().for_each(|(x, y)| {
                    match input.lines().nth(*x).and_then(|l| l.chars().nth(*y)) {
                        Some(c) if is_symbol(c) => is_valid_num = true,
                        _ => (),
                    }
                });
                s_num.push(ch);
            } else {
                if is_valid_num {
                    is_valid_num = false;
                    let num: usize = s_num.parse().unwrap_or(0);
                    nums.push(num);
                }
                s_num.clear();
            }
        }
    }
    nums.iter().sum()
    */
}

fn part_2(_input: &str) -> usize {
    // TODO
    2
}

fn main() {
    let cwd = env::current_dir().unwrap_or(PathBuf::default());
    let input_path = cwd.join("assets").join("day_3_input.txt");

    let input = fs::read_to_string(&input_path).unwrap_or(String::default());

    let part_1_result = part_1(&input);
    let part_2_result = part_2(&input);

    println!("Part 1:\t{}", part_1_result);
    println!("Part 2:\t{}", part_2_result);
}
