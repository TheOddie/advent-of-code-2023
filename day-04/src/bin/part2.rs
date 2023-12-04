use scanf::sscanf;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../res/input.txt");
    let output = solve(input);
    println!("Result: `{output:?}`!");
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    winning_nums: HashSet<u32>,
    your_nums: HashSet<u32>,
}

fn parse_nums(nums: &str) -> HashSet<u32> {
    nums.split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<_>>()
}

fn parse_card(line: &str) -> Card {
    let mut card_num = String::new();
    let mut winning_nums = String::new();
    let mut your_nums = String::new();
    sscanf!(line, "Card {}: {} | {}", card_num, winning_nums, your_nums).unwrap();
    Card {
        winning_nums: parse_nums(&winning_nums),
        your_nums: parse_nums(&your_nums),
    }
}

fn solve(input: &str) -> String {
    let scores = input
        .lines()
        .map(parse_card)
        .map(|c| c.calculate_score())
        .collect::<Vec<u32>>();

    let len = scores.len();
    let mut num_cards = vec![1; len];

    for i in 0..len {
        let score = scores[i] as usize;
        let num_c = num_cards[i];
        for j in (i + 1..i + 1 + score).filter(|&n| n < len) {
            num_cards[j] += num_c;
        }
    }
    num_cards.iter().sum::<u32>().to_string()
}

impl Card {
    fn calculate_score(&self) -> u32 {
        self.winning_nums.intersection(&self.your_nums).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = solve(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30".to_string());
    }

    #[test]
    fn test_parse_nums() {
        let result = parse_nums("24 12 26 39 19 98 74 16 82 77");
        let target = HashSet::from([24, 12, 26, 39, 19, 98, 74, 16, 82, 77]);
        assert_eq!(result, target);
    }

    #[test]
    fn test_parse_card() {
        let result = parse_card("Card   1: 24 12 26 39 19 98 74 16 82 77 | 80 11 51  1 74 60 77 68 42 35 39 78 21 12 29 19 25 98 65 91 33 17 59 24 31");
        let target = Card {
            winning_nums: HashSet::from([24, 12, 26, 39, 19, 98, 74, 16, 82, 77]),
            your_nums: HashSet::from([
                80, 11, 51, 1, 74, 60, 77, 68, 42, 35, 39, 78, 21, 12, 29, 19, 25, 98, 65, 91, 33,
                17, 59, 24, 31,
            ]),
        };
        assert_eq!(result, target);
    }
}
