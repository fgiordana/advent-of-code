use std::fs;

use anyhow::Result;


const FILEPATH: &str = "data/day_03/input.txt";


pub fn run() -> Result<()> {
    let input = fs::read_to_string(FILEPATH)?;
    println!("Part1: {}", part1(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> Result<u32> {
    let diagram = Diagram::new(input);
    let result = diagram.get_valid_parts().iter()
        .map(|p| p.id)
        .sum::<u32>();
    Ok(result)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Part {
    pub id: u32,
    pub bounds: (u32, u32),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Symbol {
    pub id: char,
    pub pos: u32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Line {
    pub parts: Vec<Part>,
    pub symbols: Vec<Symbol>,
}

impl Line {
    pub fn new(input: &str) -> Self {
        let mut parts = vec![];
        let mut symbols = vec![];
        let mut cur_start = None;
        let mut value = 0;
        for (idx, c) in input.chars().enumerate() {
            if c.is_ascii_digit() {
                // Start part if none were started
                if cur_start.is_none() {
                    cur_start = Some(idx as u32);
                }
                value = value * 10 + c.to_digit(10).unwrap();
            } else {
                // Close part if one was started
                if let Some(start) = cur_start {
                    let end = (idx - 1) as u32;
                    parts.push(Part {
                        id: value,
                        bounds: (start, end),
                    });
                    cur_start = None;
                    value = 0;
                }
                // Check if it's a symbol
                if c != '.' {
                    symbols.push(Symbol { id: c, pos: idx as u32 });
                } 
            }
        }
        // Close part if one is still open 
        if let Some(start) = cur_start {
            let end = (input.len() - 1) as u32;
            parts.push(Part {
                id: value,
                bounds: (start, end),
            });
        }

        Self {
            parts,
            symbols,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Diagram {
    pub lines: Vec<Line>,
}

impl Diagram {
    pub fn new(input: &str) -> Self {
        let lines = input.split('\n')
            .map(Line::new)
            .collect::<Vec<_>>();
        
        Self { lines }
    }

    pub fn get_valid_parts(&self) -> Vec<Part> {
        self.lines.iter()
            .enumerate()
            .flat_map(|(idx, l)| l.parts.iter().map(move |p| (idx as u32, p)))
            .filter(|(idx, p)| {
                let mut indices = vec![];
                if *idx > 0 {
                    indices.push(*idx as usize - 1);
                }
                indices.push(*idx as usize);
                if *idx < self.lines.len() as u32 - 1 {
                    indices.push(*idx as usize + 1);
                }
                indices.iter()
                    .flat_map(|&idx| self.lines[idx].symbols.iter())
                    .any(|s| is_adjacent(&p.bounds, &s.pos))
            })
            .map(|(_idx, p)| p)
            .cloned()
            .collect::<Vec<_>>()
    }
}

fn is_adjacent(bounds: &(u32, u32), pos: &u32) -> bool {
    (bounds.0 as i32 - 1) <= *pos as i32 && bounds.1 + 1 >= *pos
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = 
        "467..114..\n\
         ...*......\n\
         ..35..633.\n\
         ......#...\n\
         617*......\n\
         .....+.58.\n\
         ..592.....\n\
         ......755.\n\
         ...$.*....\n\
         .664.598..";
    
    #[test]
    fn test_part1() {
        assert_eq!(
            part1(TEST_DATA).unwrap(),
            4361
        );
    }

    #[test]
    fn test_line() {
        assert_eq!(
            Line::new(""),
            Line::default(),
        );
        assert_eq!(
            Line::new("467..114.."),
            Line { parts: vec![
                Part {
                    id: 467,
                    bounds: (0, 2),
                }, Part {
                    id: 114,
                    bounds: (5, 7),
                },
            ], symbols: vec![], }
        );
        assert_eq!(
            Line::new("...*......"),
            Line { parts: vec![], symbols: vec![Symbol {id: '*', pos: 3}]}
        );
        assert_eq!(
            Line::new(".....210................356..*.........977.68.........38.......835"),
            Line { parts: vec![
                Part { id: 210, bounds: (5, 7) },
                Part { id: 356, bounds: (24, 26) },
                Part { id: 977, bounds: (39, 41) },
                Part { id: 68, bounds: (43, 44) },
                Part { id: 38, bounds: (54, 55) },
                Part { id: 835, bounds: (63, 65)},
            ], symbols: vec![
                Symbol { id: '*', pos: 29 },
            ] }
        );
    }

    #[test]
    fn test_diagram() {
        assert_eq!(
            Diagram::new(TEST_DATA),
            Diagram { lines: vec![
                Line { parts: vec![
                    Part { id: 467, bounds: (0, 2) },
                    Part { id: 114, bounds: (5, 7) },
                ], symbols: vec![] },
                Line { parts: vec![], symbols: vec![
                    Symbol { id: '*', pos: 3 },
                ]},
                Line { parts: vec![
                    Part { id: 35, bounds: (2, 3) },
                    Part { id: 633, bounds: (6, 8) },
                ], symbols: vec![] },
                Line { parts: vec![], symbols: vec![
                    Symbol { id: '#', pos: 6 },
                ] },
                Line { parts: vec![
                    Part { id: 617, bounds: (0, 2) },
                ], symbols: vec![
                    Symbol { id: '*', pos: 3},
                ] },
                Line { parts: vec![
                    Part { id: 58, bounds: (7, 8) },
                ], symbols: vec![
                    Symbol { id: '+', pos: 5 },
                ] },
                Line { parts: vec![
                    Part { id: 592, bounds: (2, 4) },
                ], symbols: vec![] },
                Line { parts: vec![
                    Part { id: 755, bounds: (6, 8) },
                ], symbols: vec![] },
                Line { parts: vec![], symbols: vec![
                    Symbol { id: '$', pos: 3 },
                    Symbol { id: '*', pos: 5 },
                ] },
                Line { parts: vec![
                    Part { id: 664, bounds: (1, 3) },
                    Part { id: 598, bounds: (5, 7) },
                ], symbols: vec![] },
            ]}
        );
    }

    #[test]
    fn test_valid_parts() {
        let diagram = Diagram::new(TEST_DATA);
        assert_eq!(
            diagram.get_valid_parts(),
            vec![
                Part { id: 467, bounds: (0, 2) },
                Part { id: 35, bounds: (2, 3) },
                Part { id: 633, bounds: (6, 8) },
                Part { id: 617, bounds: (0, 2) },
                Part { id: 592, bounds: (2, 4) },
                Part { id: 755, bounds: (6, 8) },
                Part { id: 664, bounds: (1, 3) },
                Part { id: 598, bounds: (5, 7) },
            ]
        );
    }

    #[test]
    fn test_2() {
        const DATA: &str = 
            ".....210................356..*.........977.68.........38.......835.622.332.....*300.....131.422..............89..*.....+..........$.........\n\
             ..............14..312......+..926.....*.......529..*............*...*....*.............*......%...310.......*...835..................885....\n\
             ...416../467..........................423.....*...143...132..955...356...124.........588..947....*.....512......................134&.*......";

        let diagram = Diagram::new(DATA);
        assert_eq!(
            diagram.get_valid_parts()
                .iter()
                .map(|p| p.id)
                .collect::<Vec<_>>(),
            vec![356, 977, 835, 622, 332, 300, 131, 422, 89, 926, 529, 310, 835, 885, 467, 423, 143, 955, 356, 124, 588, 134]
        );
    }

    #[test]
    fn test_3() {
        const DATA: &str = 
            ".....210................356..*.........977.68.........38.......835\n\
             ..............14..312......+..926.....*.......529..*............*.\n\
             416...../467..........................423.....*...143...132..955..";

        let diagram = Diagram::new(DATA);
        assert_eq!(
            diagram.get_valid_parts()
                .iter()
                .map(|p| p.id)
                .collect::<Vec<_>>(),
            vec![356, 977, 835, 926, 529, 467, 423, 143, 955]
        );
    }

}
