use crate::settings::Settings;
use crate::traverse::traverse;
use eyre::bail;
use eyre::Result;
use log::debug;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use toml::{Table, Value};

fn format_string(s: &str, settings: &Settings) -> String {
    if settings.raw_output {
        s.to_string()
    } else {
        // Add surrounding quotes
        format!("\"{}\"", s)
    }
}

fn format_flat_kv(key: &str, value: &Value, settings: &Settings, level: usize) -> String {
    let formatted_key = format_string(key, &settings);
    let formatted_value = format_rec(&value, &settings, level + 1);
    format!("{}{}{}", formatted_key, settings.separator, formatted_value)
}

fn flat_format_table(table: &Table, settings: &Settings, level: usize) -> String {
    let num_items = table.len();
    match num_items {
        0 => {
            return String::from("{}");
        }
        1 => {
            let (k, v) = table.iter().next().unwrap();
            return format!("{{{}}}", format_flat_kv(&k, &v, &settings, level));
        }
        _ => {
            let mut s = String::from("{");
            let mut items = table.iter();
            let mut i = 1;
            while i < num_items {
                let (k, v) = items.next().unwrap();
                let repr = format!(
                    "{},{}",
                    format_flat_kv(&k, &v, &settings, level),
                    settings.spacing
                );
                s.push_str(&repr);
                i += 1;
            }
            let (final_k, final_v) = items.next().unwrap();
            let repr = format!("{}}}", format_flat_kv(&final_k, &final_v, &settings, level));
            s.push_str(&repr);
            return s;
        }
    }
}

fn list_format_table(table: &Table, settings: &Settings, level: usize) -> String {
    let mut s = String::new();
    for (k, v) in table.iter() {
        let repr = format!(
            "{}{}{}\n",
            format_string(k, &settings),
            settings.separator,
            format_rec(v, &settings, level + 1)
        );
        s.push_str(&repr);
    }
    s
}

fn pretty_format_table(table: &Table, settings: &Settings, level: usize) -> String {
    let mut s = String::new();
    for (k, v) in table.iter() {
        let title = format!("[{}]\n", format_string(&k, &settings));
        s.push_str(&title);
        let value = format!("{}\n", format_rec(&v, &settings, level + 1));
        s.push_str(&value)
    }
    s
}

fn format_rec(value: &Value, settings: &Settings, level: usize) -> String {
    debug!("Formatting {}", value);
    match value {
        Value::String(string) => {
            return format_string(&string, &settings);
        }
        Value::Integer(int) => {
            return int.to_string();
        }
        Value::Float(float) => {
            return float.to_string();
        }
        Value::Boolean(bool) => {
            return bool.to_string();
        }
        Value::Datetime(date) => {
            return date.to_string();
        }
        Value::Array(array) => {
            let num_items = array.len();
            match num_items {
                0 => return String::from("[]"),
                1 => {
                    let item = &array[0];
                    return format!("[{}]", format_rec(item, &settings, level + 1));
                }
                _ => {
                    let last_index = num_items - 1;
                    let mut s = String::from("[");
                    for item in &array[0..last_index] {
                        let repr = format!(
                            "{},{}",
                            format_rec(&item, &settings, level + 1),
                            settings.spacing
                        );
                        s.push_str(&repr);
                    }
                    let final_item = &array[last_index];
                    let repr = format!("{}]", format_rec(&final_item, &settings, level + 1));
                    s.push_str(&repr);
                    return s;
                }
            }
        }
        Value::Table(table) => {
            if settings.compact_output || settings.json_output {
                return flat_format_table(&table, &settings, level);
            } else {
                match level {
                    0 => {
                        //
                        return pretty_format_table(&table, &settings, level);
                    }
                    1 => {
                        return list_format_table(&table, &settings, level);
                    }
                    _ => {
                        return flat_format_table(&table, &settings, level);
                    }
                }
            }
        }
    }
}

pub fn format_value(value: &Value, settings: &Settings) -> String {
    return format_rec(&value, &settings, 0);
}
