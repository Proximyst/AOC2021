use aoc::prelude::*;

const INPUT: &str = include_str!("inputs/d01.txt");

fn main() -> Result<()> {
    let input = parse(INPUT)?;
    eprintln!("Part 1: {}", part1(&input));
    eprintln!("Part 2: {}", part2(&input));
    Ok(())
}

fn parse(i: &str) -> Result<Vec<u32>> {
    Ok(i.split("\n")
        .filter(|s| !s.is_empty())
        .map(|i| i.parse())
        .try_collect()?)
}

fn part1(i: &[u32]) -> usize {
    i.iter()
        .zip(&i[1..])
        .fold(0, |accum, (&a, &b)| if a < b { accum + 1 } else { accum })
}

fn part2(i: &[u32]) -> usize {
    let sums = i.windows(3)
        .map(|window| window.iter().sum())
        .collect_vec();
    part1(&sums)
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(
            "199
200
208
210
200
207
240
269
260
263",
        )
        .unwrap(),
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263],
    );
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        5
    );
}
