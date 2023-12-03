use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = env::current_dir()
        .expect("Unable to open cwd")
        .join("assets")
        .join("day_1_input.txt");

    let file1 = File::open(&file_path).expect("Unable to open file");
    let file2 = File::open(&file_path).expect("Unable to open file");

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let part_one_sum = part_one(reader1);
    let part_two_sum = part_two(reader2);

    println!("Part One Solution:\t{}", part_one_sum);
    println!("Part Two Solution:\t{}", part_two_sum);
}

/// # Day 1: Trebuchet?!
/// Something is wrong with global snow production, and you've been selected to take a look.
/// The Elves have even given you a map; on it, they've used stars to mark the top fifty locations
/// that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations, you need to check
/// all *fifty stars* by December 25th.
///
/// Collect stars by solving puzzles.  Two puzzles will be made available on each day in the Advent
/// Calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants *one star*.
/// Good luck!
///
/// You try to ask why they can't just use a weather machine ("not powerful enough") and where
/// they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of
/// questions") and hang on did you say the sky ("of course, where do you think snow comes from")
/// when you realize that the Elves are already loading you into a trebuchet ("please hold still, we
/// need to strap you in").
///
/// As they're making the final adjustments, they discover that their calibration document (your
/// puzzle input) has been *amended* by a very young Elf who was apparently just excited to show off
/// her art skills. Consequently, the Elves are having trouble reading the values on the document.
///
/// The newly-improved calibration document consists of lines of text; each line originally contained
/// a specific *calibration value* that the Elves now need to recover. On each line, the calibration
/// value can be found by combining the first digit and the last digit (in that order) to form a
/// single *two-digit number*.
///
/// # Example
///
/// ```
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ```
///
/// In this example, the calibration values for these four lines are `12`, `38`, `15`, and `17`.
/// Adding these together produces `142`.
///
/// Consider your entire calibration document. *What is the sum of all of the calibration values?*
fn part_one(reader: BufReader<File>) -> usize {
    reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let mut char_nums: VecDeque<_> = line.split(|c: char| !c.is_digit(10))
                .filter(|s| !s.is_empty())
                .flat_map(|s| s.chars())
                .collect();

            let first = char_nums.pop_front().unwrap_or_default();
            let last = char_nums.pop_back().unwrap_or(first);
            let concatenated = format!("{}{}", first, last);
            concatenated.parse::<usize>().ok()
        }).sum()
}

/// Your calculation isn't quite right. It looks like some of the digits are actually *spelled out
/// with letters:* `one`, `two`, `three`, `four`, `five`, `six`, `seven`, `eight`, and `nine` *also*
/// count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each
/// line.
///
/// # Example
///
/// ```
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ```
///
/// In this example, the calibration values are `29`, `83`, `13`, `24`, `42`, `14`, and `76`. Adding
/// these together produces `281`.
///
/// *What is the sum of all of the calibration values?*
fn part_two(reader: BufReader<File>) -> usize {
    let map_nums = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    let mut num_map: HashMap<&str, char> = HashMap::new();
    for (k, v) in map_nums.into_iter() {
        num_map.insert(k, v);
    }

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let mut v: VecDeque<char> = VecDeque::new();
            for (i, c) in line.char_indices() {
                if c.is_digit(10) {
                    v.push_back(c);
                } else if let substring = &line[i..line.len()] {
                    for (key, val) in num_map.iter() {
                        if substring.starts_with(key) {
                            v.push_back(*val);
                        }
                    }
                }
            }
            let first = v.pop_front().unwrap_or_default();
            let last = v.pop_back().unwrap_or(first);
            let concatenated = format!("{}{}", first, last);
            concatenated.parse::<usize>().ok()
        }).sum()
}