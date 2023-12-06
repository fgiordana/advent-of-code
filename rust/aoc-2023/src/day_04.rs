use std::fs;

use anyhow::Result;



const FILEPATH: &str = "data/day_04/input.txt";

pub fn run() -> Result<()> {
    let input = fs::read_to_string(FILEPATH)?;
    println!("Part1: {}", part1(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> Result<u32> {
    let result = input.split('\n')
        .map(Card::new)
        .map(|c| c.score())
        .sum::<u32>();

    Ok(result)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn new(input: &str) -> Self {
        let (_, card) = parser::card(input).unwrap();
        card
    }

    pub fn score(&self) -> u32 {
        let n = self.numbers
            .iter()
            .filter(|n| self.winners.contains(n))
            .count();
        match n {
            0 => 0,
            _ => 2u32.pow(n as u32 - 1)
        }
    }
}


mod parser {
    use nom::{
        IResult, 
        bytes::complete::tag, 
        character::complete::{char, u32, multispace0}, 
        sequence::{separated_pair, preceded, delimited}, 
        combinator::map, 
        error::ParseError, multi::many0 
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

    pub fn card(input: &str) -> IResult<&str, Card> {
        map(
            separated_pair(card_id, char(':'), card_body),
            |(id, (winners, numbers))| Card { id, winners, numbers }
        )(input)
    }

    fn card_id(input: &str) -> IResult<&str, u32> {
        preceded(tag("Card"), ws(u32))(input)
    }

    fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
        many0(ws(u32))(input)
    }

    fn card_body(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
        separated_pair(numbers, tag("|"), numbers)(input)
    }


    #[cfg(test)]
    mod test {

        use super::*;

        #[test]
        fn test_card_id() {
            assert_eq!(
                card_id("Card 1"),
                Ok(("", 1))
            );
        }

        #[test]
        fn test_numbers() {
            assert_eq!(
                numbers(" 41 48 83 86 17 "),
                Ok(("", vec![41, 48, 83, 86, 17]))
            );
        }

        #[test]
        fn test_card_body() {
            assert_eq!(
                card_body(" 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
                Ok(("", (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])))
            );
        }

        #[test]
        fn test_card() {
            assert_eq!(
                card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
                Ok(("", Card { 
                    id: 1, 
                    winners: vec![41, 48, 83, 86, 17], 
                    numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
                }))
            )
        }
    }
}


#[cfg(test)]
mod test {

    use super::*;

    const TEST_DATA: &str = 
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    
    #[test]
    fn test_part1() {
        assert_eq!(
            part1(TEST_DATA).unwrap(),
            13
        );
    }

    #[test]
    fn test_parse_card() {
        assert_eq!(
            Card::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Card {
                id: 1,
                winners: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }

    #[test]
    fn test_card_score() {
        assert_eq!(
            Card::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").score(),
            8
        );
    }
}
