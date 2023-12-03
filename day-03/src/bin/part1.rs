use std::str::Chars;

fn main() {
    let input = include_str!("../../res/input.txt");
    let output = solve(input);
    println!("Result: `{output:?}`!");
}

#[derive(Debug)]
struct PartNumber {
    num: u32,
    counted: bool,
}

#[derive(Default, Debug)]
struct GridInfo {
    part_numbers: Vec<PartNumber>,
    symbol_locations: Vec<(usize, usize)>,
    grid: Vec<Vec<Option<usize>>>,
}

#[derive(Default, PartialEq, Eq, Debug)]
struct LineInfo {
    part_numbers: Vec<u32>,
    symbol_locations: Vec<usize>,
    row: Vec<Option<usize>>,
}

fn read_num(first_ch: char, chars: &mut Chars<'_>) -> (u32, usize) {
    let mut num = first_ch.to_digit(10).unwrap();
    let mut jumps = 0;
    loop {
        match chars.clone().next() {
            None => return (num, jumps),
            Some(ch) => match ch {
                '0'..='9' => {
                    num *= 10;
                    num += ch.to_digit(10).unwrap();
                    chars.next();
                    jumps += 1;
                }
                _ => {
                    return (num, jumps);
                }
            },
        }
    }
}

fn parse_line(line: &str) -> LineInfo {
    let mut line_info = LineInfo::default();
    let mut chars = line.chars();
    let mut x = 0;
    while let Some(ch) = chars.next() {
        match ch {
            '.' => line_info.row.push(None),
            num if ch.is_numeric() => {
                let (part_num, dx) = read_num(num, &mut chars);
                let idx = line_info.part_numbers.len();
                line_info.part_numbers.push(part_num);
                x += dx;
                line_info
                    .row
                    .append(&mut (0..=dx).map(|_| idx).map(Some).collect::<Vec<_>>())
            }
            _symb => {
                line_info.symbol_locations.push(x);
                line_info.row.push(None);
            }
        }
        x += 1;
    }
    line_info
}

fn parse(input: &str) -> GridInfo {
    let mut grid_info = GridInfo::default();
    let mut part_count = 0;
    for (y, line) in input.lines().enumerate() {
        let mut line_info = parse_line(line);
        line_info.row.iter_mut().for_each(|opt_n| {
            if let Some(n) = opt_n {
                *n += part_count
            }
        });
        part_count += line_info.part_numbers.len();
        grid_info.grid.push(line_info.row);
        grid_info.part_numbers.append(
            &mut line_info
                .part_numbers
                .iter()
                .map(|&n| PartNumber {
                    num: n,
                    counted: false,
                })
                .collect::<Vec<_>>(),
        );
        grid_info.symbol_locations.append(
            &mut line_info
                .symbol_locations
                .iter()
                .map(|&x| (x, y))
                .collect::<Vec<_>>(),
        )
    }
    grid_info
}

fn solve(input: &str) -> String {
    let mut grid = parse(input);
    let width = grid.grid[0].len();
    let height = grid.grid.len();
    let mut total = 0;
    for &(x, y) in grid.symbol_locations.iter() {
        let x_iter = if x == 0 { x..=x + 1 } else if x == width - 1 { x - 1..=x } else { x - 1..=x + 1 };
        let y_iter = if y == 0 { y..=y + 1 } else if y == height - 1 { y - 1..=y } else { y - 1..=y + 1 };
        for iy in y_iter {
            for ix in x_iter.clone() {
                if let Some(part_index) = grid.grid[iy][ix] {
                    if !grid.part_numbers[part_index].counted {
                        grid.part_numbers[part_index].counted = true;
                        total += grid.part_numbers[part_index].num;
                    }
                }
            }
        }
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = solve(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "4361".to_string());
    }

    #[test]
    fn test_read_num() {
        let mut chars = "67..114..".chars();
        let first_num = read_num('4', &mut chars);
        let remaining_chars = chars.collect::<String>();
        assert_eq!(first_num, (467, 2));
        assert_eq!(remaining_chars, "..114..".to_string());
    }

    #[test]
    fn test_parse_line() {
        let result = parse_line("467..+.58.");
        assert_eq!(
            result,
            LineInfo {
                part_numbers: vec![467, 58],
                symbol_locations: vec![5],
                row: vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    None,
                    None,
                    None,
                    Some(1),
                    Some(1),
                    None
                ]
            }
        );
    }

    #[test]
    fn test_parse() {
        let grid = parse(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        dbg!(grid);
    }
}
