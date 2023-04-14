mod clause;
mod clause_parser;

use clause::{display_clauses, Clause};
use clause_parser::parse_input;
use std::fmt::Write;
use std::{fmt, rc::Rc};

const WRITE_FAIL_MESSAGE: &'static str =
    "Failed to write result string. This is probably not recoverable.";

#[derive(PartialEq, Eq, Hash)]
struct ClauseEdge {
    from: (Rc<Clause>, Rc<Clause>),
    to: Rc<Clause>,
    clashing_literal: String,
}

impl fmt::Display for ClauseEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Resolve {} and {} on literal <span class='literal'>{}</span> to obtain {}",
            self.from.0, self.from.1, self.clashing_literal, self.to
        )
    }
}

pub struct ClauseGraph {
    node_groups: Vec<Vec<Rc<Clause>>>,
    edges: Vec<Vec<ClauseEdge>>,
}

pub fn try_parse_input(input: &str) -> Result<(), String> {
    parse_input(input).map(|_| ())
}

impl ClauseGraph {
    pub fn new(input: &str) -> Result<Self, String> {
        parse_input(input).map(|start_clauses| {
            let mut result = ClauseGraph {
                node_groups: vec![start_clauses
                    .into_iter()
                    .map(|clause| Rc::new(clause))
                    .collect::<Vec<_>>()],
                edges: vec![Vec::new()],
            };
            result.resolve();
            result
        })
    }

    fn contains_clause(&self, clause: &Clause) -> bool {
        for node_group in self.node_groups.iter() {
            for node in node_group.iter() {
                if node.as_ref() == clause {
                    return true;
                }
            }
        }
        false
    }

    fn resolve(&mut self) {
        loop {
            let mut new_node_group = Vec::new();
            let mut new_edges = Vec::new();

            for nodes in self.node_groups.iter() {
                for clause in nodes {
                    for other_clause in self.node_groups.last().unwrap() {
                        for (resolvent, clashing_literal) in clause.as_ref().try_clash(other_clause)
                        {
                            let new_clause = Rc::new(resolvent);
                            if (!self.contains_clause(new_clause.as_ref())
                                && !new_node_group
                                    .iter()
                                    .any(|new_node| *new_node == new_clause))
                                || (new_clause.is_empty()
                                    && !new_edges.iter().any(|edge: &ClauseEdge| {
                                        edge.clashing_literal == clashing_literal
                                    }))
                            {
                                new_node_group.push(new_clause.clone());
                                new_edges.push(ClauseEdge {
                                    from: (clause.clone(), other_clause.clone()),
                                    to: new_clause,
                                    clashing_literal,
                                });
                            }
                        }
                    }
                }
            }
            if new_node_group.is_empty() && new_edges.is_empty() {
                break;
            }
            self.node_groups.push(new_node_group);
            self.edges.push(new_edges);
        }
    }

    fn check_satisfiability(&self) -> Option<usize> {
        for (index, node_group) in (0..self.node_groups.len()).zip(self.node_groups.iter()) {
            if node_group.iter().any(|node| node.is_empty()) {
                return Some(index);
            }
        }
        None
    }

    pub fn get_resolvents(&self) -> String {
        let mut result = String::new();
        let mut node_groups_iterator = (0..self.node_groups.len()).zip(self.node_groups.iter());
        write!(&mut result,
            "Given: {}\n",
            display_clauses(node_groups_iterator.next().expect("No clauses were added to ClauseGraph. This is a bug and should be reported to the developer.").1)
        ).expect(WRITE_FAIL_MESSAGE);
        for (iteration, node_group) in node_groups_iterator {
            write!(
                &mut result,
                "Iteration {}: {}\n",
                iteration,
                display_clauses(node_group)
            )
            .expect(WRITE_FAIL_MESSAGE);
        }
        result
    }

    pub fn get_solution(&self) -> String {
        let mut result = String::new();
        if let Some(index) = self.check_satisfiability() {
            let mut all_options = Vec::new();
            for empty_clause in self.node_groups[index]
                .iter()
                .filter(|node| node.is_empty())
            {
                let mut steps = vec![self
                    .edges
                    .last()
                    .expect("No edges added to ClauseGraph. This is a bug and should be reported to the developer.")
                    .iter()
                    .filter(|edge| Rc::ptr_eq(&edge.to, empty_clause))
                    .next()
                    .expect("Empty clause has no edge leading to it. This is a bug and should be reported to the developer.")];
                let mut manual_iterator = 0;
                while manual_iterator < steps.len() {
                    let (parent1, parent2) = &steps[manual_iterator].from;
                    for edge_group in self.edges.iter() {
                        for edge in edge_group {
                            if *parent1 == edge.to && !steps.contains(&edge) {
                                steps.push(edge);
                                break;
                            }
                        }
                    }
                    if *parent1 != *parent2 {
                        for edge_group in self.edges.iter() {
                            for edge in edge_group {
                                if *parent2 == edge.to && !steps.contains(&edge) {
                                    steps.push(edge);
                                    break;
                                }
                            }
                        }
                    }
                    manual_iterator += 1;
                }
                all_options.push(steps);
            }
            all_options.sort_by(|x, y| x.len().cmp(&y.len()));
            let shortest_steps = all_options.first().expect("Formula is unsatisfiable but there are no steps to get the empty set. This is a bug and should be reported to the developer.").len();
            all_options.retain(|steps| steps.len() == shortest_steps);
            print!("The formula is unsatisfiable! ");
            if all_options.len() == 1 {
                write!(
                    &mut result,
                    "The quickest way to get the empty set is to do these {} steps:\n",
                    shortest_steps
                )
                .expect(WRITE_FAIL_MESSAGE);
                for step in all_options.first().expect("Formula is unsatisfiable but there are no steps to get the empty set. This is a bug and should be reported to the developer.").iter().rev() {
                    write!(&mut result, "{}\n", step).expect(WRITE_FAIL_MESSAGE);
                }
            } else {
                write!(
                    &mut result,
                    "There are {} ways to get the empty set, all taking {} steps:\n",
                    all_options.len(),
                    shortest_steps
                )
                .expect(WRITE_FAIL_MESSAGE);
                for (iteration, steps) in (1..).zip(all_options.iter()) {
                    write!(&mut result, "\nOption {}:\n", iteration).expect(WRITE_FAIL_MESSAGE);
                    for step in steps.iter().rev() {
                        write!(&mut result, "{}\n", step).expect(WRITE_FAIL_MESSAGE);
                    }
                }
            }
        } else {
            write!(
                &mut result,
                "The formula is satisfiable! Take these steps to get all possible clauses:\n"
            )
            .expect(WRITE_FAIL_MESSAGE);
            let mut edges_iter = self.edges.iter();
            edges_iter.next();
            for (iteration, edge_group) in (1..).zip(edges_iter) {
                write!(&mut result, "\nIteration {}:\n", iteration).expect(WRITE_FAIL_MESSAGE);
                for edge in edge_group {
                    write!(&mut result, "{}\n", edge).expect(WRITE_FAIL_MESSAGE);
                }
            }
            write!(
                &mut result,
                "\nAt the end you should get these {} clauses:\n",
                self.node_groups
                    .iter()
                    .map(|node_group| node_group.len())
                    .sum::<usize>()
            )
            .expect(WRITE_FAIL_MESSAGE);
            write!(&mut result, "{}", self.get_resolvents()).expect(WRITE_FAIL_MESSAGE);
        }
        result = result
            .split("\n")
            .map(|line| {
                let mut modified_line = "<div class='result-line'>".to_owned();
                if line.is_empty() {
                    modified_line.push_str("&nbsp;");
                } else {
                    modified_line.push_str(line);
                }
                modified_line.push_str("</div>");
                modified_line
            })
            .collect();
        result
    }
}
