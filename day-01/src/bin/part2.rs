fn main() {
    let input = include_str!("../../res/input.txt");
    let output = solve(input);
    println!("Result: `{output:?}`!");
}

static NUMS_AS_WORDS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn map_nums(line: &str) -> String {
    for (i, _) in line.chars().enumerate() {
        let iter = line.chars().skip(i);

        let mut tmp = None;
        for &(word, num) in NUMS_AS_WORDS.iter() {
            let len_word = word.len();
            if iter.clone().take(len_word).collect::<String>() == word {
                tmp = Some(num);
                break;
            }
        }

        if let Some(a) = tmp {
            return format!(
                "{}{}{}",
                line.chars().take(i).collect::<String>(),
                a,
                map_nums(&line.chars().skip(i + 1).collect::<String>())
            );
        }
    }
    line.into()
}

fn solve(input: &str) -> String {
    let mapped_input = input.lines().map(map_nums).collect::<Vec<String>>();
    mapped_input
        .iter()
        .map(|e| e.chars().filter(|ch| ch.is_numeric()))
        .map(|chs| (chs.clone(), chs.clone().rev()))
        .map(|(mut left, mut right)| (left.next().unwrap(), right.next().unwrap()))
        .map(|(a, b)| format!("{}{}", a, b))
        .map(|s| s.parse::<i64>().unwrap())
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = solve(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string());
    }

    #[test]
    fn test2() {
        let result = map_nums("oneight")
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>();
        assert_eq!(result, "18".to_string());
    }
}
