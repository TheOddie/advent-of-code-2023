fn main() {
    let input = include_str!("../../res/input.txt");
    let output = solve(input);
    println!("Result: `{output:?}`!");
}

fn solve(input: &str) -> String {
    input
        .lines()
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
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, "142".to_string());
    }
}
