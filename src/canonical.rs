use crate::calculator::Calculator;
use std::collections::{HashMap, HashSet};

fn calculate_with_assignment(expr: &str, assignments: &HashMap<char, bool>) -> bool {
    let mut replaced_expr = expr.replace("T", "true").replace("F", "false");

    for (var_name, value) in assignments.iter() {
        let var_str = if *value { "true" } else { "false" };
        replaced_expr = replaced_expr.replace(&var_name.to_string(), var_str);
    }

    Calculator::build(&replaced_expr).eval()
}

pub fn truth_table(expr: &str) -> Vec<(HashMap<char, bool>, bool)> {
    let variables: HashSet<char> = expr
        .chars()
        .filter(|c| c.is_alphabetic() && *c != 'F' && *c != 'T')
        .collect();
    let mut truth_values = Vec::new();
    let num_combinations = 1 << variables.len();

    for i in 0..num_combinations {
        let mut var_assignments = HashMap::new();
        for (index, var) in variables.iter().enumerate() {
            var_assignments.insert(*var, (i & (1 << index)) != 0);
        }
        let result = calculate_with_assignment(&expr.to_string(), &var_assignments);
        truth_values.push((var_assignments, result));
    }

    truth_values
}

pub fn disjunctive_normal_form(truth_table: &[(HashMap<char, bool>, bool)]) -> String {
    let mut disjunctions = Vec::new();
    for (assignments, result) in truth_table.iter() {
        if *result {
            let mut conjunction = Vec::new();
            for (var, &value) in assignments {
                if value {
                    conjunction.push(var.to_string());
                } else {
                    conjunction.push(format!("¬{}", var));
                }
            }
            disjunctions.push("(".to_string() + &conjunction.join(" ∧ ") + ")");
        }
    }

    disjunctions.join(" ∨ ")
}

pub fn conjunctive_normal_form(truth_table: &[(HashMap<char, bool>, bool)]) -> String {
    let mut conjunctions = Vec::new();
    for (assignments, result) in truth_table.iter() {
        if !*result {
            let mut disjunction = Vec::new();
            for (var, &value) in assignments {
                if value {
                    disjunction.push(format!("¬{}", var));
                } else {
                    disjunction.push(var.to_string());
                }
            }
            conjunctions.push("(".to_string() + &disjunction.join(" ∧ ") + ")");
        }
    }

    conjunctions.join(" ∧ ")
}

pub fn print_truth_table(table: &[(HashMap<char, bool>, bool)]) {
    if let Some((var_assignments, _)) = table.first() {
        let vars: Vec<char> = var_assignments.keys().cloned().collect();

        for var in &vars {
            print!("{}\t", var);
        }
        println!("Result");

        for (var_assignments, result) in table {
            for var in &vars {
                let value = var_assignments.get(var).unwrap();
                print!("{}\t", value);
            }
            println!("{}", result);
        }
    } else {
        println!("Table is empty!");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_truth_table() {
        let result = truth_table("((A ∧ B) → C) ↔ A");
        println!("{:?}", result);
    }

    #[test]
    fn test_disjunctive() {
        let table = truth_table("((A ∧ B) → C) ↔ A");
        let result = disjunctive_normal_form(&table);
        println!("DNF: {:?}", result);
    }

    #[test]
    fn test_conjunctive() {
        let table = truth_table("((A ∧ B) → C) ↔ A");
        let result = conjunctive_normal_form(&table);
        println!("CNF: {:?}", result);
    }

    #[test]
    fn test_print_table() {
        let table = truth_table("((A ∧ B) → C) ↔ A");
        print_truth_table(&table);
    }
}