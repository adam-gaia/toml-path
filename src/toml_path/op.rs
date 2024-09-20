use eyre::bail;
use eyre::Result;
use std::fmt;
use toml::{Table, Value};
use winnow::ascii::alphanumeric1;
use winnow::ascii::dec_int;
use winnow::ascii::space0;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::separated;
use winnow::combinator::separated_pair;
use winnow::combinator::seq;
use winnow::error::StrContext;
use winnow::error::StrContextValue;
use winnow::prelude::*;
use winnow::token::take_while;

// TODO: add winnnow contexts for better error messages to all parsers

/// Range between two numbers.
/// Negative numbers index from the end, where '-1' is the final item.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Range {
    start: isize,
    end: isize,
}

impl Range {
    /// Create a new Range from start and end indexes.
    pub fn new(start: isize, end: isize) -> Self {
        Self { start, end }
    }

    /// Generate all values in the range. Length is needed to compute the positive equivalent of negative indexes.
    /// TODO: this needs some tests!
    pub fn gen_range_indexes(&self, length: usize) -> Result<Vec<usize>> {
        let length = length as isize;
        let mut indexes: Vec<usize> = Vec::new();
        // TODO: we could consider rewriting this match block where each branch returns the range and that range gets plugged into a single for loop at the end
        match (self.start, self.end) {
            (start, end) if start >= 0 && end < 0 => {
                // Start positive/zero, end negative
                let end_fixed = (length + end) as usize;
                let start = start as usize;
                if end_fixed < start {
                    bail!("end < start ({:?})", self);
                }

                for n in start..=end_fixed {
                    indexes.push(n);
                }
            }
            (start, end) if start < 0 && end >= 0 => {
                // Start negative and end positive/zero
                let start_fixed = (length + start) as usize;
                let end = end as usize;
                if end < start_fixed {
                    bail!("end < start ({:?})", self);
                }

                for n in start_fixed..end {
                    indexes.push(n);
                }
            }
            (start, end) if start >= 0 && end >= 0 => {
                // Start and end both positive/zero
                let start = start as usize;
                let end = end as usize;
                if end < start {
                    bail!("end < start ({:?})", self);
                }

                for n in start..end {
                    indexes.push(n);
                }
            }
            (start, end) if start < 0 && end < 0 => {
                // Start and end both negative
                let start_fixed = (length + start) as usize;
                let end_fixed = (length + end) as usize;
                if end_fixed < start_fixed {
                    bail!("end < start ({:?})", self);
                }

                for n in start_fixed..end_fixed {
                    indexes.push(n);
                }
            }
            _ => {
                unreachable!("If this happens I have an error in my range logic");
            }
        }
        Ok(indexes)
    }
}

fn number(s: &mut &str) -> PResult<isize> {
    dec_int.parse_next(s)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Index {
    Number(isize),
    Range(Range),
}

fn range(s: &mut &str) -> PResult<Range> {
    separated_pair(number, space_colon_space, number)
        .map(|(start, end)| Range { start, end })
        .parse_next(s)
}

fn index_range(s: &mut &str) -> PResult<Index> {
    range.map(|r| Index::Range(r)).parse_next(s)
}

fn index_number(s: &mut &str) -> PResult<Index> {
    number.map(|x| Index::Number(x)).parse_next(s)
}

fn index(s: &mut &str) -> PResult<Index> {
    // I think that range needs to be checked before number
    alt((index_range, index_number)).parse_next(s)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Op {
    /// "."
    Dot,

    /// Alphanumeric identifier
    Name(String),

    /// List of indicies and/or index ranges
    /// [1, 2:4, 5]
    /// TODO: rename this to 'Index' or something
    BracketIndex(Vec<Index>),

    /// List of alphanumeric identifiers
    /// ["foo", "bar"]
    /// TODO: rename this to 'NameIndex' or something
    BracketName(Vec<String>),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Op::Dot => String::from("."),
            Op::Name(string) => {
                format!("Op::Name({:?})", string)
            }
            Op::BracketIndex(indexes) => {
                format!("Op::BracketIndex({:?})", indexes)
            }
            Op::BracketName(names) => {
                format!("Op::BracketName({:?})", names)
            }
        };

        write!(f, "{}", repr)
    }
}

fn dot(s: &mut &str) -> PResult<Op> {
    ".".map(|_| Op::Dot).parse_next(s)
}

fn name_str(s: &mut &str) -> PResult<String> {
    alphanumeric1.map(|s: &str| s.to_string()).parse_next(s)
}

fn name(s: &mut &str) -> PResult<Op> {
    name_str.map(|name| Op::Name(name)).parse_next(s)
}

fn quoted_name(s: &mut &str) -> PResult<String> {
    delimited("\"", name_str, "\"").parse_next(s)
}

fn comma_space(s: &mut &str) -> PResult<()> {
    let _ = ",".parse_next(s)?;
    let _ = space0.parse_next(s)?;
    Ok(())
}

fn space_colon_space(s: &mut &str) -> PResult<()> {
    let _ = space0.parse_next(s)?;
    let _ = ":".parse_next(s)?;
    let _ = space0.parse_next(s)?;
    Ok(())
}

fn bracket_name_list(s: &mut &str) -> PResult<Op> {
    let list = separated(1.., quoted_name, comma_space)
        .map(|list| Op::BracketName(list))
        .parse_next(s)?;
    // Ignore possible trailing comma and space
    let _ = opt(comma_space).parse_next(s)?;
    Ok(list)
}

fn bracket_index_list(s: &mut &str) -> PResult<Op> {
    let list = separated(1.., index, comma_space)
        .map(|list| Op::BracketIndex(list))
        .parse_next(s)?;
    // Ignore possible trailing comma and space
    let _ = opt(comma_space).parse_next(s)?;
    Ok(list)
}

fn open_bracket_space(s: &mut &str) -> PResult<()> {
    let _ = "[".parse_next(s)?;
    let _ = space0.parse_next(s)?;
    Ok(())
}

fn close_bracket_space(s: &mut &str) -> PResult<()> {
    let _ = "]".parse_next(s)?;
    let _ = space0.parse_next(s)?;
    Ok(())
}

fn bracket(s: &mut &str) -> PResult<Op> {
    delimited(
        open_bracket_space,
        alt((bracket_index_list, bracket_name_list)),
        close_bracket_space,
    )
    .parse_next(s)
}

pub fn op(s: &mut &str) -> PResult<Op> {
    alt((dot, name, bracket)).parse_next(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! s {
        ($literal:expr) => {
            String::from($literal)
        };
    }

    // TODO: add tests for non-happy paths

    #[test]
    fn test_positive_number() {
        let mut input = "1";
    }

    #[test]
    fn test_zero() {
        let mut input = "0";
        let expected = Ok(0);
        let result = number(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_negative_number() {
        let mut input = "-1";
        let expected = Ok(-1);
        let result = number(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_space_colon_space_1() {
        let mut input = ":";
        let result = space_colon_space(&mut input);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_space_colon_space_2() {
        let mut input = " :";
        let result = space_colon_space(&mut input);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_space_colon_space_3() {
        let mut input = ": ";
        let result = space_colon_space(&mut input);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_space_colon_space_4() {
        let mut input = " : ";
        let result = space_colon_space(&mut input);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_range() {
        let mut input = "1:3";
        let expected = Ok(Range { start: 1, end: 3 });
        let result = range(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_range_list_one_item() {
        let mut input = "1:4";
        let expected = Ok(Op::BracketIndex(vec![Index::Range(Range::new(1, 4))]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_range_list_one_item_trailing_comma() {
        let mut input = "1:4,";
        let expected = Ok(Op::BracketIndex(vec![Index::Range(Range::new(1, 4))]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_range_list_multiple_items() {
        let mut input = "1:2, 2:3, 3, 4";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Range(Range::new(1, 2)),
            Index::Range(Range::new(2, 3)),
            Index::Number(3),
            Index::Number(4),
        ]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_range_list_no_space() {
        let mut input = "1:1,2:2,3:3";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Range(Range::new(1, 1)),
            Index::Range(Range::new(2, 2)),
            Index::Range(Range::new(3, 3)),
        ]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_range_list_trailing_comma() {
        let mut input = "0:1,1:2,2:3,";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Range(Range::new(0, 1)),
            Index::Range(Range::new(1, 2)),
            Index::Range(Range::new(2, 3)),
        ]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_range_list_negatives() {
        let mut input = "0:1, 0:-2,-2, -3";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Range(Range::new(0, 1)),
            Index::Range(Range::new(0, -2)),
            Index::Number(-2),
            Index::Number(-3),
        ]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_num_list_one_item() {
        let mut input = "1";
        let expected = Ok(Op::BracketIndex(vec![Index::Number(1)]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_num_list_one_item_trailing_comma() {
        let mut input = "1,";
        let expected = Ok(Op::BracketIndex(vec![Index::Number(1)]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_num_list_multiple_items() {
        let mut input = "1, 2, 3";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Number(1),
            Index::Number(2),
            Index::Number(3),
        ]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_num_list_no_space() {
        let mut input = "1,2,3";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Number(1),
            Index::Number(2),
            Index::Number(3),
        ]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_num_list_trailing_comma() {
        let mut input = "1,2,3,";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Number(1),
            Index::Number(2),
            Index::Number(3),
        ]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_num_list_negatives() {
        let mut input = "1, -2,-3";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Number(1),
            Index::Number(-2),
            Index::Number(-3),
        ]));
        let result = bracket_index_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_name_list_one_item() {
        let mut input = "\"1\"";
        let expected = Ok(Op::BracketName(vec![s!("1")]));
        let result = bracket_name_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_name_list_one_item_trailing_comma() {
        let mut input = "\"1\",";
        let expected = Ok(Op::BracketName(vec![s!("1")]));
        let result = bracket_name_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_name_list_multiple_items() {
        let mut input = "\"1\", \"2\", \"3\"";
        let expected = Ok(Op::BracketName(vec![s!("1"), s!("2"), s!("3")]));
        let result = bracket_name_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_name_list_no_space() {
        let mut input = "\"1\",\"2\",\"3\"";
        let expected = Ok(Op::BracketName(vec![s!("1"), s!("2"), s!("3")]));
        let result = bracket_name_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_bracket_name_list_trailing_comma() {
        let mut input = "\"1\",\"2\",\"3\",";
        let expected = Ok(Op::BracketName(vec![s!("1"), s!("2"), s!("3")]));
        let result = bracket_name_list(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_op_dot() {
        let mut input = ".";
        let expected = Ok(Op::Dot);
        let result = op(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_op_name() {
        let mut input = "name";
        let expected = Ok(Op::Name(String::from("name")));
        let result = op(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_op_bracket_name_list() {
        let mut input = "[\"name\"]";
        let expected = Ok(Op::BracketName(vec![String::from("name")]));
        let result = op(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }
    #[test]
    fn test_op_bracket_num_list() {
        let mut input = "[1, 2, 3]";
        let expected = Ok(Op::BracketIndex(vec![
            Index::Number(1),
            Index::Number(2),
            Index::Number(3),
        ]));
        let result = op(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_op_bracket_range() {
        let mut input = "[1:4]";
        let expected = Ok(Op::BracketIndex(vec![Index::Range(Range {
            start: 1,
            end: 4,
        })]));
        let result = op(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }
}
