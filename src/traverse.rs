use crate::toml_path::{Index, Op, TomlPath};
use eyre::bail;
use eyre::Result;
use log::debug;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use toml::{Table, Value};
use winnow::ascii::alphanumeric1;
use winnow::ascii::dec_int;
use winnow::ascii::space0;
use winnow::combinator::delimited;
use winnow::combinator::repeat;
use winnow::combinator::separated;
use winnow::combinator::separated_pair;
use winnow::combinator::seq;
use winnow::prelude::*;
use winnow::token::take_while;

pub fn traverse(value: &Value, path: &[Op]) -> Result<Value> {
    let current_op = &path[0];
    let num_ops = path.len();
    match value {
        Value::String(string) => {
            if num_ops > 1 {
                bail!(
                    "Hit the end of toml tree (string: '{}') but path has more parts left: {:?}",
                    string,
                    path[1..].to_vec()
                );
            }
            return Ok(value.clone());
        }
        Value::Integer(int) => {
            if num_ops > 1 {
                bail!(
                    "Hit the end of toml tree (integer: {}) but path has more parts left: {:?}",
                    int,
                    path[1..].to_vec()
                );
            }
            return Ok(value.clone());
        }
        Value::Float(float) => {
            if num_ops > 1 {
                bail!(
                    "Hit the end of toml tree (float: {}) but path has more parts left: {:?}",
                    float,
                    path[1..].to_vec()
                );
            }
            return Ok(value.clone());
        }
        Value::Boolean(bool) => {
            if num_ops > 1 {
                bail!(
                    "Hit the end of toml tree (bool: {}) but path has more parts left: {:?}",
                    bool,
                    path[1..].to_vec()
                );
            }
            return Ok(value.clone());
        }
        Value::Datetime(date) => {
            if num_ops > 1 {
                bail!(
                    "Hit the end of toml tree (datetime: '{}') but path has more parts left: {:?}",
                    date,
                    path[1..].to_vec()
                );
            }
            return Ok(value.clone());
        }
        Value::Array(array) => match current_op {
            Op::Dot => {
                if num_ops == 1 {
                    return Ok(value.clone());
                }
                return traverse(&value, &path[1..]);
            }
            Op::Name(name) => {
                bail!("Cannot index array with string ({:?})", name);
            }
            Op::BracketIndex(indexes) => {
                let num_items = array.len();
                let mut filtered_values: Vec<Value> = Vec::new();
                for index in indexes {
                    match index {
                        Index::Number(i_signed) => {
                            let i_unsigned = if *i_signed < 1 {
                                (num_items as isize + i_signed) as usize
                            } else {
                                *i_signed as usize
                            };
                            let Some(item) = array.get(i_unsigned) else {
                                bail!("No item at index {} in array ({:?})", i_unsigned, array);
                            };
                            filtered_values.push(item.clone());
                        }
                        Index::Range(range) => {
                            for i in range.gen_range_indexes(num_items)? {
                                let Some(item): Option<&Value> = array.get(i) else {
                                    bail!(
                                        "No item at index {} (from range {:?}) in array ({:?})",
                                        i,
                                        range,
                                        array
                                    );
                                };
                                filtered_values.push(item.clone());
                            }
                        }
                    }
                }
                let subset = Value::Array(filtered_values);
                if num_ops == 1 {
                    return Ok(subset);
                }
                return traverse(&subset, &path[1..]);
            }
            Op::BracketName(names) => {
                bail!("Cannot index array with strings ({:?})", names);
            }
        },
        Value::Table(table) => match current_op {
            Op::Dot => {
                if num_ops == 1 {
                    return Ok(value.clone());
                }
                return traverse(&value, &path[1..]);
            }
            Op::Name(name) => {
                let Some(section) = table.get(name) else {
                    bail!("Could not find key '{:?}' in table ({:?})", name, table);
                };
                if num_ops == 1 {
                    return Ok(section.clone());
                }
                return traverse(section, &path[1..]);
            }
            Op::BracketIndex(indexes) => {
                bail!("Cannot index table with indexes ({:?})", indexes)
            }
            Op::BracketName(names) => {
                let mut filtered_values: Vec<Value> = Vec::new();

                for name in names {
                    let Some(section) = table.get(name) else {
                        bail!(
                            "Could not find key '{:?}' (from keys ({:?}) in table ({:?})",
                            name,
                            names,
                            table
                        );
                    };
                    filtered_values.push(section.clone());
                }

                let subset = Value::Array(filtered_values);
                if num_ops == 1 {
                    return Ok(subset);
                }
                return traverse(&subset, &path[1..]);
            }
        },
    }
}
