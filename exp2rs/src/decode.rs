use super::*;
use regex::Regex;
use types::*;

const SYNTAX_ERROR: &str = "Syntax Error";

lazy_static::lazy_static! {
    static ref COMMENT_REGEX: Regex = Regex::new(r"(--[^\n]*\n)|\(\*[\s\S]*?\*\)|\n").unwrap();
}

fn remove_comments_and_newline(code: &str) -> String {
    COMMENT_REGEX.replace_all(code, "").into()
}

pub fn decode(code: &str) -> Result<Vec<Schema>, &str> {
    let mut res = Vec::new();
    let code = remove_comments_and_newline(code);
    let mut current_schema: Option<Schema> = None;
    let mut current_type: Option<Type> = None;
    for line in code.split(";") {
        let line = line.trim();
        let (command, type_name) = get_command_typename(line);
        if command == "SCHEMA" {
            current_schema = Some(Schema {
                name: type_name,
                types: Vec::new(),
            });
            continue;
        }
        if command == "END_SCHEMA" {
            match current_schema.take() {
                Some(schema) => res.push(schema),
                None => return Err(SYNTAX_ERROR),
            }
        }
        if command == "ENTITY" {
            current_type = Some(Type::Entity {
                name: type_name,
                members: Vec::new(),
            });
            continue;
        }
        if command == "END_ENTITY" {
            match (current_schema.as_mut(), current_type.take()) {
                (Some(schema), Some(tmp_type)) => schema.types.push(tmp_type),
                _ => return Err(SYNTAX_ERROR),
            }
            continue;
        }
        if current_type.is_none() {
            continue;
        }
        match current_type.as_mut().unwrap() {
            Type::Entity { members, .. } => {
                let (type_name, variables, optional) = declare_variables(line);
                for variable in variables {
                    members.push(MemberVariant {
                        name: variable.trim().to_string(),
                        type_name: type_name.to_string(),
                        optional,
                    });
                }
            }
            _ => {}
        }
    }
    Ok(res)
}

fn get_command_typename(line: &str) -> (String, String) {
    let mut words = line.split_whitespace();
    let command = if let Some(word) = words.next() {
        word.chars().map(|c| c.to_ascii_uppercase()).collect()
    } else {
        String::new()
    };
    let type_name = if let Some(word) = words.next() {
        word.to_string()
    } else {
        String::new()
    };
    (command, type_name)
}

fn declare_variables(line: &str) -> (&str, Vec<&str>, bool) {
    println!("{}", line);
    let colon_divided = line.split(":").collect::<Vec<_>>();
    let variables = colon_divided[0].split(",").collect::<Vec<_>>();
    let the_type = colon_divided[1].split_whitespace().collect::<Vec<_>>();
    let idx = the_type.len() - 1;
    (the_type[idx], variables, idx == 1)
}
