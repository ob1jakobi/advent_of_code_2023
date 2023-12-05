use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

/// You're launching high into the atmosphere!  The apex of your trajectory just barely reaches the
/// surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's
/// quite cold, but you don't see much snow. A Elf runs over to greet you.
///
/// The Elf explains that you've arrived at *Snow Island* and apologizes for the lack of snow. He'll
/// be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't
/// get many visitors up here; would you like to play a game in the meantime?
///
/// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
/// Each time you play this game, he will hide a secret number of cubes of each color in the bag,
/// and your goal is to figure out information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab
/// a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a
/// few times per game.
///
/// You play several games and record the information from each game (your puzzle input). Each game
/// is listed with its ID number (like the `11` in `Game 1: ...`) followed by a semicolon-separated
/// list of subsets of cubes that were revealed from the bag (like `3 red, 5 green, 4 blue`).
///
/// # Example
///
/// ```
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
/// ```
///
/// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first
/// set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue
/// cubes; the third set is only 2 green cubes.
///
/// The Elf would first like to know which games would have been possible if the bag contained
/// *only 12 red cubes, 13 green cubes, and 14 blue cubes*?
///
/// In the example above, games 1, 2, and 5 would have been *possible* if the bag had been loaded
/// with that configuration.  However, game 3 would have been *impossible* because at one point the
/// Elf showed you 20 red cubes at once; similarly, game 4 would also have been *impossible* because
/// the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have
/// been possible, you get *8*.
///
/// Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
/// 13 green cubes, and 14 blue cubes. *What is the sum of the IDs of those games?*
fn part_1(reader: BufReader<File>) -> usize {
    let color_regex = Regex::new(r"(\d+) (\w+)").expect("Bad color regex pattern");

    reader
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(line_num, line)| {
            let line = line.as_str();
            let game_num = line_num + 1;
            let has_valid_color_counts = color_regex
                .captures_iter(line)
                .map(|color_capture| {
                    let count: usize = color_capture[1].parse().unwrap_or(0);
                    match color_capture[2].to_string().as_str() {
                        "blue" => count <= 14,
                        "green" => count <= 13,
                        "red" => count <= 12,
                        _ => false,
                    }
                }).fold(true, |acc, new_count_valid| acc && new_count_valid);
            if has_valid_color_counts {
                game_num
            } else {
                0
            }
        }).sum()
}

/// The Elf says they've stopped producing snow because they aren't getting any *water*! He isn't
/// sure why the water stopped; however, he can show you how to get the water source to check it out
/// for yourself. It's just up ahead!
///
/// As you continue your walk, the Elf poses a second question: in each game you played, what is the
/// *fewest number of cubes of each color* that could have been in the bag to make the game possible?
///
/// Again consider the example games from earlier:
///
/// # Example
///
/// ```
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
/// ```
///
/// * In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If
///   any color had been one fewer cube, the game would have been impossible.
/// * Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
/// * Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
/// * Game 4 required at least 14 red, 3 green, and 15 blue cubes.
/// * Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
///
/// The *power* of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied
/// together. The power of the minimum set of cubes in game 1 is `48`. In games 2-5 it was `12`,
/// `1560`, `630`, and `36`, respectively.  Adding up these five powers produces the sum `2286`.
///
/// For each game, find the minimum set of cubes that must have been present. *What is the sum of
/// the power of these sets?*
fn part_2(reader: BufReader<File>) -> usize {
    let color_regex = Regex::new(r"(\d+) (\w+)").expect("Bad color regex pattern");
    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let line = line.as_str();

            let (blue, green, red) = color_regex
                .captures_iter(line)
                .map(|color_capture| {
                    let count: usize = color_capture[1].parse().unwrap_or(0);
                    match color_capture[2].to_string().as_str() {
                        "blue" => (count, 0, 0),
                        "green" => (0, count, 0),
                        "red" => (0, 0, count),
                        _ => (0, 0, 0),
                    }
                }).fold((0, 0, 0), |acc, new_count| {
                let b = max(acc.0, new_count.0);
                let g = max(acc.1, new_count.1);
                let r = max(acc.2, new_count.2);
                (b, g, r)
            });

            blue * green * red
        }).sum()
}

fn main() {
    let cwd = env::current_dir().expect("Unable to get cwd");
    let file_path = cwd.join("assets").join("day_2_input.txt");

    let day_2_file_1 = File::open(&file_path).expect("Unable to open file");
    let day_2_file_2 = File::open(&file_path).expect("Unable to open file");

    let reader1 = BufReader::new(day_2_file_1);
    let reader2 = BufReader::new(day_2_file_2);

    let part_1_sum = part_1(reader1);
    let part_2_sum = part_2(reader2);

    println!("Part 1:\t{}", part_1_sum);
    println!("Part 2:\t{}", part_2_sum);
}