use aoc::prelude::*;

const INPUT: &str = include_str!("inputs/d02.txt");

fn main() -> Result<()> {
    let input = parse(INPUT)?;
    eprintln!("Part 1: {}", part1(&input));
    eprintln!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(i: &[Instruction]) -> i64 {
    let t = i
        .iter()
        .fold((0i64, 0i64), |(x, y), instr| match instr.direction {
            Direction::Horizontal => (x + instr.magnitude as i64, y),
            Direction::Vertical => (x, y + instr.magnitude as i64),
        });
    t.0 * t.1
}

fn part2(i: &[Instruction]) -> i64 {
    let t = i.iter().fold((0i64, 0i64, 0i64), |(x, y, aim), instr| {
        match instr.direction {
            Direction::Horizontal => (
                x + instr.magnitude as i64,
                y + (instr.magnitude as i64 * aim),
                aim,
            ),
            Direction::Vertical => (x, y, aim + instr.magnitude as i64),
        }
    });
    t.0 * t.1 // t.2 is only for calculations
}

fn parse(i: &str) -> Result<Vec<Instruction>> {
    Ok(i.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(" "))
        .filter_map(|o| o)
        .map(|(direction, magnitude)| {
            magnitude
                .parse::<i32>()
                .wrap_err("invalid magnitude")
                .map(|m| (direction, m))
                .and_then(|(direction, magnitude)| match direction {
                    "forward" => Ok(Instruction::horizontal(magnitude)),
                    "up" => Ok(Instruction::vertical(-magnitude)),
                    "down" => Ok(Instruction::vertical(magnitude)),
                    _ => Err(err!("Invalid direction: {}", direction)),
                })
        })
        .try_collect()?)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Instruction {
    magnitude: i32,
    direction: Direction,
}

impl Instruction {
    fn new(magnitude: i32, direction: Direction) -> Self {
        Self {
            magnitude,
            direction,
        }
    }

    fn vertical(magnitude: i32) -> Self {
        Self::new(magnitude, Direction::Vertical)
    }

    fn horizontal(magnitude: i32) -> Self {
        Self::new(magnitude, Direction::Horizontal)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Vertical,
    Horizontal,
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2"
        )
        .unwrap(),
        vec![
            Instruction::horizontal(5),
            Instruction::vertical(5),
            Instruction::horizontal(8),
            Instruction::vertical(-3),
            Instruction::vertical(8),
            Instruction::horizontal(2),
        ],
    );
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&[
            Instruction::horizontal(5),
            Instruction::vertical(5),
            Instruction::horizontal(8),
            Instruction::vertical(-3),
            Instruction::vertical(8),
            Instruction::horizontal(2),
        ]),
        15 * 10,
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part1(&[
            Instruction::horizontal(5),
            Instruction::vertical(5),
            Instruction::horizontal(8),
            Instruction::vertical(-3),
            Instruction::vertical(8),
            Instruction::horizontal(2),
        ]),
        15 * 60,
    );
}
