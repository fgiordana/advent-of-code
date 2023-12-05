use std::fs;

use anyhow::Result;

pub struct Setup {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

const SETUP: Setup = Setup {
    red: 12,
    green: 13,
    blue: 14,
};

const FILEPATH: &str = "data/day_02/input.txt";


pub fn run() -> Result<()> {
    let input = fs::read_to_string(FILEPATH)?;
    println!("Part1: {}", part1(&SETUP, &input)?);
    println!("Part2: {}", part2(&input)?);
    Ok(())
}

pub fn part1(setup: &Setup, input: &str) -> Result<u32> {
    let sum = input.split('\n')
        .map(|line| parse_game(line).unwrap())
        .filter(|g| {
            let max = g.max();
            max.0 <= setup.red && max.1 <= setup.green && max.2 <= setup.blue
        })
        .map(|g| g.id)
        .sum();
    Ok(sum)
}

pub fn part2(input: &str) -> Result<u32> {
    let sum = input.split('\n')
        .map(|line| parse_game(line).unwrap()) 
        .map(|g| g.power())
        .sum();
    Ok(sum)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<(u32, u32, u32)>,
}

impl Game {
    pub fn max(&self) -> (u32, u32, u32) {
        self.sets.iter()
            .fold((0, 0, 0), |acc, set| {
                (
                    acc.0.max(set.0),
                    acc.1.max(set.1),
                    acc.2.max(set.2)
                )
            })
    }

    pub fn power(&self) -> u32 {
        let max = self.max();
        max.0 * max.1 * max.2
    }
}

fn parse_game(input: &str) -> Result<Game> {
    let (_, game) = parser::game(input).unwrap();
    Ok(game.clone())
} 

mod parser {
    use nom::{
        IResult, 
        bytes::complete::tag, 
        character::complete::{char, u32, multispace0}, 
        sequence::{separated_pair, preceded, delimited, terminated}, 
        combinator::{map, opt}, 
        error::ParseError, multi::separated_list0, branch::alt 
    };
    use super::*;

    /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and 
    /// trailing whitespace, returning the output of `inner`.
    fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(
            multispace0,
            inner,
            multispace0
        )
    }

    pub fn game(input: &str) -> IResult<&str, Game> {
        map(
            separated_pair(game_id, char(':'), terminated(sets, opt(char('\n')))),
            |(id, sets)| Game { id, sets }
        )(input)
    }

    fn game_id(input: &str) -> IResult<&str, u32> {
        preceded(tag("Game"), ws(u32))(input)
    }

    fn sets(input: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
        separated_list0(char(';'), set)(input)
    }
   
    fn set(input: &str) -> IResult<&str, (u32, u32, u32)> {
        map(
            separated_list0(tag(","), alt((red, green, blue))),
            |values| values.iter()
                .fold((0, 0, 0), |acc, num| {
                    (
                        acc.0 + num.0,
                        acc.1 + num.1,
                        acc.2 + num.2
                    )
                })
        )(input)
    }

    fn red(input: &str) -> IResult<&str, (u32, u32, u32)> {
        map(
            terminated(ws(u32), tag("red")),
            |r| (r, 0, 0)
        )(input)
    }

    fn green(input: &str) -> IResult<&str, (u32, u32, u32)> {
        map(
            terminated(ws(u32), tag("green")),
            |g| (0, g, 0)
        )(input)
    }

    fn blue(input: &str) -> IResult<&str, (u32, u32, u32)> {
        map(
            terminated(ws(u32), tag("blue")),
            |b| (0, 0, b)
        )(input)
    }



    #[cfg(test)]
    mod test {

        use super::*;

        #[test]
        fn test_game_id() {
            assert_eq!(
                game_id("Game 1"),
                Ok(("", 1))
            );
        }

        #[test]
        fn test_red() {
            assert_eq!(
                red("4 red"),
                Ok(("", (4, 0, 0)))
            );
            assert_eq!(
                red(" 4  red "),
                Ok((" ", (4, 0, 0)))
            );
        }

        #[test]
        fn test_set() {
            assert_eq!(
                set(" 1 red, 2 green, 6 blue"),
                Ok(("", (1, 2, 6)))
            );
            assert_eq!(
                set("1 red, 2 blue, 6 green"),
                Ok(("", (1, 6, 2)))
            );
            assert_eq!(
                set("4 red, 3 blue"),
                Ok(("", (4, 0, 3)))
            );
            assert_eq!(
                set("3 blue, 4 red"),
                Ok(("", (4, 0, 3)))
            );
            assert_eq!(
                set("2 green, 6 blue"),
                Ok(("", (0, 2, 6)))
            );
            assert_eq!(
                set(" 2 green"),
                Ok(("", (0, 2, 0)))
            );
            assert_eq!(
                set("3 blue, 4 red, 1 green"),
                Ok(("", (4, 1, 3)))
            );
        }

        #[test]
        fn test_sets() {
            assert_eq!(
                sets("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
                Ok(("", vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]))
            );
            assert_eq!(
                sets(" 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
                Ok(("", vec![(6, 3, 1), (1, 2, 2)]))
            );
        }

        #[test]
        fn test_game() {
            assert_eq!(
                game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n"), 
                Ok(("", Game { id: 1, sets: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]}))
            );
            assert_eq!(
                game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
                Ok(("", Game { id: 5, sets: vec![(6, 3, 1), (1, 2, 2)]})) 
            );
        }
    }
}


#[cfg(test)]
mod test {

    use super::*;

    const TEST_DATA: &str = 
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
         Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
         Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
         Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
         Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    
    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&SETUP, TEST_DATA).unwrap(),
            8,
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(TEST_DATA).unwrap(),
            2286
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n").unwrap(),
            Game{
                id: 1,
                sets: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]
            }
        );
        assert_eq!(
            parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n").unwrap(),
            Game{
                id: 2,
                sets: vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]
            }
        );
        assert_eq!(
            parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n").unwrap(),
            Game{
                id: 3,
                sets: vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]
            }
        );
        assert_eq!(
            parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n").unwrap(),
            Game{
                id: 4,
                sets: vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]
            }
        );
        assert_eq!(
            parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(),
            Game{
                id: 5,
                sets: vec![(6, 3, 1), (1, 2, 2)]
            }
        );
    }

    #[test]
    fn test_max() {
        assert_eq!(
            Game{
                id: 1,
                sets: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]
            }.max(),
            (4, 2, 6)
        );
    }

    #[test]
    fn test_power() {
        assert_eq!(
            Game{
                id: 1,
                sets: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]
            }.power(),
            48
        ); 
    }
}
