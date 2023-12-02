use scanf::sscanf;

fn main() {
    let input = include_str!("../../res/input.txt");
    let output = solve(input);
    println!("Result: `{output:?}`!");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: u32,
    r: u32,
    g: u32,
    b: u32,
}

fn parse(line: &str) -> Game {
    // example
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    let mut iter = line.split_terminator(':');
    let mut id: u32 = 0;
    dbg!(line); // print when in debug so i know why the parser crashed XD
    sscanf!(iter.next().unwrap(), "Game {}", id).unwrap();
    let mut game = Game { id, r: 0, g: 0, b: 0 };
    for hand in iter.next().unwrap().split_terminator(';') { /* ` 1 blue, 2 green` */
        for col in hand.split_terminator(',') { /* ` 1 blue`` */
            let mut colour = "".to_string();
            let mut count = 0;
            // beware the leading space in ` 1 blue`
            sscanf!(col, " {} {}", count, colour).unwrap();
            match colour.as_str() {
                "red" => game.r = game.r.max(count),
                "green" => game.g = game.g.max(count),
                "blue" => game.b = game.b.max(count),
                other => unreachable!("Got an illegal colour! `{other}`")
            }
        }
    }
    game
}

impl Game {
    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

fn solve(input: &str) -> String {
    input
        .lines()
        .map(parse)
        .map(|g| g.power())
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = solve(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286".to_string());
    }

    #[test]
    fn test_parse() {
        let result = parse(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
        );
        assert_eq!(result, Game { id: 2, r: 1, g: 3, b: 4 });
    }

    #[test]
    fn test_power() {
        let result = parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").power();
        assert_eq!(result, 48);
    }
}
