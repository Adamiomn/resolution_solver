use itertools::Itertools;
use std::{fmt, rc::Rc};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Literal {
    name: String,
    is_negated: bool,
}

impl Literal {
    pub fn new(name: String, is_negated: bool) -> Self {
        Literal { name, is_negated }
    }

    fn is_opposite(&self, other_literal: &Literal) -> bool {
        self.name == other_literal.name && self.is_negated == !other_literal.is_negated
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negated {
            write!(f, "¬{}", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[derive(Eq, Hash)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl PartialEq for Clause {
    fn eq(&self, other: &Self) -> bool {
        self.literals
            .iter()
            .all(|literal| other.literals.contains(literal))
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clause_string: Vec<String> = self
            .literals
            .iter()
            .map(|variable| format!("{}", variable))
            .collect();
        write!(f, "{{{}}}", clause_string.join(", "))
    }
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Clause {
            literals: literals.into_iter().unique().collect_vec(),
        }
    }

    pub fn insert(&mut self, literal: Literal) {
        if !self.literals.contains(&literal) {
            self.literals.push(literal);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn try_clash(&self, other_clause: &Clause) -> Vec<(Clause, String)> {
        let mut resolvent_clauses = Vec::new();
        for literal in self.literals.iter() {
            if other_clause
                .literals
                .iter()
                .any(|other_literal| literal.is_opposite(other_literal))
            {
                let mut set1 = self.literals.clone();
                set1.retain(|resolvent_literal| *resolvent_literal != *literal);
                let mut set2 = other_clause.literals.clone();
                set2.retain(|resolvent_literal| {
                    resolvent_literal.name != literal.name
                        || resolvent_literal.is_negated == literal.is_negated
                });
                set1.append(&mut set2);
                resolvent_clauses.push((Clause::new(set1), literal.name.clone()));
            }
        }
        resolvent_clauses
    }
}

pub fn display_clauses(clauses: &Vec<Rc<Clause>>) -> String {
    let clauses_string: Vec<String> = clauses.iter().map(|clause| clause.to_string()).collect();
    clauses_string.join(", ")
}
