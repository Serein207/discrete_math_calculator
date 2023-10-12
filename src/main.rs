use crate::canonical::{
    conjunctive_normal_form, disjunctive_normal_form, print_truth_table, truth_table,
};

mod calculator;
mod canonical;
mod lexer;
mod parser;

fn main() {
    let expr = "((A ∧ B) → C) ↔ A";
    let truth_table = truth_table(&expr);
    print_truth_table(&truth_table);
    let dnf = disjunctive_normal_form(&truth_table);
    let cnf = conjunctive_normal_form(&truth_table);
    println!("DNF: {}\nCNF: {}", dnf, cnf);
}