use super::clause::{Clause, Literal};

pub fn parse_input(input: &str) -> Result<Vec<Clause>, String> {
    if input.len() == 0 {
        return Err("No formula given.".to_owned());
    }
    let mut brace_counter = 0;
    let mut max_brace_counter = 0;
    for chars in input.chars() {
        if chars == '{' {
            brace_counter += 1;
        } else if chars == '}' {
            brace_counter -= 1;
        }
        max_brace_counter = max_brace_counter.max(brace_counter);
        if brace_counter < 0 {
            return Err("Curly braces do not match".to_owned());
        }
    }
    if brace_counter != 0 {
        return Err("Curly braces do not match".to_owned());
    }
    if max_brace_counter == 0 {
        return Err("There are no curly braces. Each clause must use exactly one set of curly braces, e.g. {p, q}.".to_owned());
    }
    if max_brace_counter > 2 {
        return Err("There are too many curly braces. Each clause must use exactly one set of curly braces, e.g. {p, q}. All the clauses may be enclosed in one set of curly braces (or not). E.g. {{p, q}, {r, s}} or {p, q}, {r, s}".to_owned());
    }
    let mut trimmed_input = input.trim();
    if max_brace_counter == 2 {
        trimmed_input = &trimmed_input[1..trimmed_input.len() - 1];
        trimmed_input = trimmed_input.trim();
    }

    let mut clauses = Vec::new();
    let mut start = None;
    for (index, chars) in trimmed_input.char_indices() {
        if chars == '{' {
            start = Some(index + 1);
        } else if chars == '}' {
            let raw_clause = &trimmed_input[start.expect("Found closing curly brace before opening curly brace. This should be caught earlier on and is a bug in the code. Please report to the developer.")..index];
            match parse_clause(raw_clause) {
                Ok(parsed_clause) => clauses.push(parsed_clause),
                Err(message) => return Err(message),
            }
        }
        if start.is_none() {
            if chars != ' ' && chars != ',' {
                return Err("Clauses may only be separated by comma and/or whitespace. E.g. {p, q} , {r, s} is valid, as is {p, q}{r, s}".to_owned());
            }
        }
    }
    Ok(clauses)
}

fn parse_clause(clause_string: &str) -> Result<Clause, String> {
    let mut clause = Clause::new(Vec::new());
    for raw_variable in clause_string.split(',') {
        match parse_variable(raw_variable) {
            Ok(variable) => clause.insert(variable),
            Err(message) => return Err(format!("Failed to parse clause. {}", message)),
        };
    }
    Ok(clause)
}

fn is_negation_symbol(negation_string: &str) -> bool {
    return negation_string == "!"
        || negation_string == "~"
        || negation_string == "¬"
        || negation_string == "not"
        || negation_string == "\\neg";
}

fn parse_variable(variable_string: &str) -> Result<Literal, String> {
    let trimmed_variable = variable_string.trim();
    let number_of_whitespaces = trimmed_variable
        .chars()
        .filter(|character| character.is_whitespace())
        .count();
    let mut is_negated = false;
    let mut name = "".to_owned();
    if number_of_whitespaces > 0 {
        let mut variable_parts = trimmed_variable.splitn(2, char::is_whitespace);
        if let Some(first_part) = variable_parts.next() {
            if is_negation_symbol(first_part) {
                is_negated = true;
            } else {
                return Err(format!("Invalid negation symbol: {}. For reference, the variable substring parsed was {}", first_part, trimmed_variable));
            }
        } else {
            return Err(format!("Failed to parse variable, literal was empty. For reference, the variable substring parsed was {}", trimmed_variable));
        }
        let second_part = variable_parts.next().expect("Failed to extract variable name after getting negation symbol. This is a bug and should be reported to the developer.");
        let trimmed_name = second_part.trim();
        if trimmed_name
            .chars()
            .filter(|character| character.is_whitespace())
            .count()
            > 0
        {
            return Err(format!("Variable name contains whitespaces. For reference, the variable name was extracted as {} and the variable substring parsed was {}", trimmed_name, trimmed_variable));
        }
        name = trimmed_name.to_owned();
    } else {
        let mut characters = trimmed_variable.chars().peekable();
        let first_character = characters.peek().expect("Failed to get negation symbol, string was empty. This is a bug and should be reported to the developer.");
        if is_negation_symbol(&first_character.to_string()) {
            is_negated = true;
            characters.next();
        }
        name.push_str(&characters.collect::<String>());
        if name.is_empty() {
            return Err(format!("Failed to parse variable, literal consisted of only a negation symbol, missing a variable. For reference, the variable substring parsed was {}", trimmed_variable));
        }
    }
    Ok(Literal::new(name, is_negated))
}
