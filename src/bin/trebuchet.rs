use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
fn main() {
    let file_path = env::current_dir()
        .expect("Unable to open cwd")
        .join("assets")
        .join("day_1_input.txt");
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let sum: usize = reader.lines()
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
        }).sum();

    println!("{}", sum);

}