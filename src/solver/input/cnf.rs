use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::usize;

use logos::*;

use crate::Clause;
use crate::ParsingError;
use crate::Solver;
use crate::Var;

use super::*;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum CNF {
    #[regex("c.*")]
    Comment,
    #[regex("p +cnf +[0-9]+ +[0-9]+ *")]
    Problem,
    #[regex(" *([+-]{0, 1}[0-9]+ +)+0")]
    Clauses,
    #[regex("%\n0")]
    StmtEnd,
}

// Entry cnf
pub fn open_dimacs_cnf(path: PathBuf) -> Result<Solver, Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let mut lexer = CNF::lexer(source.as_str());

    let mut clause: Vec<Clause> = vec![];
    let mut vars: Vec<Var> = vec![];

    loop {
        use CNF::*;
        match lexer.next() {
            Some(Ok(t)) if t == Comment => continue,
            Some(Ok(t)) if t == Problem => process_problem(&mut clause, &mut vars, lexer.slice())?,
            Some(Ok(t)) if t == Clauses => process_clauses(&mut clause, &vars, lexer.slice())?,
            Some(Ok(t)) if t == StmtEnd => continue,

            None => break,

            Some(Err(_)) => return Err(ParsingError::unexpected_token(lexer.slice()).into()),
            Some(Ok(t)) => unreachable!("Unused token type: {:?}", t),
        }
    }

    let mut solver = Solver::new();

    solver.vars = vars;
    solver.clause = Clause::And(clause);

    Ok(solver)
}

fn process_problem(
    clause: &mut Vec<Clause>,
    vars: &mut Vec<Var>,
    slice: &str,
) -> Result<(), Box<dyn Error>> {
    // Guard
    if !vars.is_empty() {
        return Err(ParsingError::mul_definition().into());
    }

    // Split matched string in white spaces.
    let mut token = slice.split_whitespace();

    // Ignoring first two values
    assert!(token.next().expect(LEXER_CONSTRAINS) == "p");
    assert!(token.next().expect(LEXER_CONSTRAINS) == "cnf");

    // Set vars capacity.
    let vars_capacity = token.next().expect(LEXER_CONSTRAINS);
    let vars_capacity: usize = vars_capacity.parse()?;
    vars.reserve(vars_capacity);

    // Fill vars
    for index in 0..vars_capacity {
        let var = Var::new(index);
        vars.push(var);
    }

    // Set clause capacity.
    let clause_capacity = token.next().expect(LEXER_CONSTRAINS);
    let clause_capacity: usize = clause_capacity.parse()?;
    clause.reserve(clause_capacity);

    Ok(())
}

fn process_clauses(
    clause: &mut Vec<Clause>,
    vars: &Vec<Var>,
    slice: &str,
) -> Result<(), Box<dyn Error>> {
    if vars.is_empty() {
        return Err(ParsingError::definition_not_found().into());
    }

    use std::cmp::Ordering::*;
    use Clause::*;

    let mut subclause: Vec<Clause> = vec![];
    let tokens = slice.split_whitespace();

    for token in tokens {
        let lit: isize = token.parse()?;

        match lit.cmp(&0) {
            Less => {
                let name = -lit as usize;
                let var = vars[name - 1];
                subclause.push(Not(var));
            }

            Greater => {
                let name = lit as usize;
                let var = vars[name - 1];
                subclause.push(Idn(var));
            }

            Equal => break,
        }
    }

    if subclause.len() == 1 {
        let lit = subclause.pop().expect("Checked Above");
        clause.push(lit);

        return Ok(());
    }

    subclause.shrink_to_fit();
    clause.push(Or(subclause));

    Ok(())
}
