use eyre::Result;
use std::str::FromStr;
use thiserror::Error;
use winnow::combinator::repeat;
use winnow::prelude::*;

mod op;
use op::op;
pub use op::Index;
pub use op::Op;

/// TODO: doc comments
/// Impls std::str::FromStr for convenience
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TomlPath {
    parts: Vec<Op>,
}

impl TomlPath {
    pub fn parts(&self) -> &[Op] {
        &self.parts
    }
}

fn path_parts(s: &mut &str) -> PResult<Vec<Op>> {
    repeat(1.., op).parse_next(s)
}

fn toml_path(s: &mut &str) -> PResult<TomlPath> {
    path_parts.map(|parts| TomlPath { parts }).parse_next(s)
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum TomlPathError {
    #[error("Unable to parse input str")]
    UnableToParse,
}

impl FromStr for TomlPath {
    type Err = TomlPathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml_path.parse(s).map_err(|_| TomlPathError::UnableToParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_path_parts() {
        let mut input = ".";
        let expected = Ok(vec![Op::Dot]);
        let result = path_parts.parse_next(&mut input);
        assert_eq!(expected, result);
        assert_eq!("", input);
    }

    #[test]
    fn test_toml_path_dot() {
        let input = ".";
        let expected = Ok(TomlPath {
            parts: vec![Op::Dot],
        });
        let result = TomlPath::from_str(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_toml_path_dot_range() {
        let input = ".[1:3]";
        let expected = Ok(TomlPath {
            parts: vec![
                Op::Dot,
                Op::BracketIndex(vec![Index::Range(Range::new(1, 3))]),
            ],
        });
        let result = TomlPath::from_str(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_toml_path_dot_list() {
        let input = ".[1, 2, 3]";
        let expected = Ok(TomlPath {
            parts: vec![
                Op::Dot,
                Op::BracketIndex(vec![Index::Number(1), Index::Number(2), Index::Number(3)]),
            ],
        });
        let result = TomlPath::from_str(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_toml_path_dot_name() {
        let input = ".foo";
        let expected = Ok(TomlPath {
            parts: vec![Op::Dot, Op::Name(String::from("foo"))],
        });
        let result = TomlPath::from_str(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_toml_path_dot_names() {
        let input = ".foo.bar.baz.qux";
        let expected = Ok(TomlPath {
            parts: vec![
                Op::Dot,
                Op::Name(String::from("foo")),
                Op::Dot,
                Op::Name(String::from("bar")),
                Op::Dot,
                Op::Name(String::from("baz")),
                Op::Dot,
                Op::Name(String::from("qux")),
            ],
        });
        let result = TomlPath::from_str(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_toml_path_dot_bracket_name() {
        let input = ".[\"foo\"]";
        let expected = Ok(TomlPath {
            parts: vec![Op::Dot, Op::BracketName(vec![String::from("foo")])],
        });
        let result = TomlPath::from_str(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_toml_path_dot_bracket_range() {
        let input = ".[1:3]";
        let expected = Ok(TomlPath {
            parts: vec![
                Op::Dot,
                Op::BracketIndex(vec![Index::Range(Range::new(1, 3))]),
            ],
        });
        let result = TomlPath::from_str(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_toml_path_dot_bracket_number() {
        let input = ".[1]";
        let expected = Ok(TomlPath {
            parts: vec![Op::Dot, Op::BracketIndex(vec![Index::Number(1)])],
        });
        let result = TomlPath::from_str(input);
        assert_eq!(expected, result);
    }
}
