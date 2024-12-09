use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::usize;

use logos::*;

use crate::Clause;
use crate::ParsingError;
use crate::ProblemType;
use crate::Solver;
use crate::Var;

use super::*;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum SAT {
    #[regex("c.*")]
    Comment,
    #[regex("p +sat([ex]|ex){0,1} +[0-9]+ *")]
    Problem,
    #[regex(r"(([\*\+=]|xor){0,1}\()|\)")]
    Clauses,
    #[regex("[+-]{0,1}[0-9]+")]
    Litrals,
}

pub fn open_dimacs_sat(path: PathBuf) -> Result<Solver, Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let mut lexer = SAT::lexer(source.as_str());

    let mut pt: Option<ProblemType> = None;

    let mut clause: Option<Clause> = None;
    let mut vars: Vec<Var> = vec![];

    let mut lit_path: Vec<usize> = vec![];

    loop {
        use ProblemType::*;
        use SAT::*;
        match lexer.next() {
            // Process Comment
            Some(Ok(t)) if t == Comment => continue,

            // Process Problem (Should only run once)
            Some(Ok(t)) if t == Problem => pt = process_problem(&mut vars, lexer.slice())?,

            // Process Clauses (Only one will for entire duartion for parsing)
            Some(Ok(t)) if t == Clauses && pt == Some(Sat) => {
                clause = process_sat(&mut lit_path, clause, lexer.slice())?
            }
            Some(Ok(t)) if t == Clauses && pt == Some(Sate) => {
                clause = process_sate(&mut lit_path, clause, lexer.slice())?
            }
            Some(Ok(t)) if t == Clauses && pt == Some(Satx) => {
                clause = process_satx(&mut lit_path, clause, lexer.slice())?
            }
            Some(Ok(t)) if t == Clauses && pt == Some(Satex) => {
                clause = process_satex(&mut lit_path, clause, lexer.slice())?
            }

            // Process Litrals
            Some(Ok(t)) if t == Litrals => {
                clause = process_litrals(clause, &vars, &lit_path, lexer.slice())?
            }

            // Process End of file
            None => break,

            // Unexpected Tokens
            Some(Err(_)) => return Err(ParsingError::unexpected_token(lexer.slice()).into()),

            // Unreachable
            Some(Ok(t)) => unreachable!("Found unused token type: {:?}", t),
        }
    }

    if clause.is_none() {
        return Err(Box::from("Empty File"));
    }

    let mut solver = Solver::new();

    solver.clause = clause.expect("Checked Above");
    solver.vars = vars;

    Ok(solver)
}

fn process_litrals(
    clause: Option<Clause>,
    vars: &Vec<Var>,
    lit_path: &Vec<usize>,
    slice: &str,
) -> Result<Option<Clause>, Box<dyn Error>> {
    let lit: isize = slice.parse().expect(LEXER_CONSTRAINS);

    use std::cmp::Ordering::*;
    let lit = match lit.cmp(&0) {
        Less => {
            let index = -lit as usize;
            Not(vars[index - 1])
        }
        Equal => {
            return Err(Box::from("Lit Cannot be zero"));
        }
        Greater => {
            let index = lit as usize;
            Idn(vars[index - 1])
        }
    };

    let mut clause = clause.expect("Todo");
    let mut subclause = clause.inner_clauses().expect("Due to process");

    for index in lit_path.iter() {
        let temp = &mut subclause[*index];
        let temp = temp.inner_clauses().expect("Due to Process");
        subclause = temp;
    }

    subclause.push(lit);

    Ok(Some(clause))
}

fn process_problem(
    vars: &mut Vec<Var>,
    slice: &str,
) -> Result<Option<ProblemType>, Box<dyn Error>> {
    if vars.len() != 0 {
        return clause_before_definition();
    }

    let mut token = slice.split_whitespace();

    // Ignore first token
    assert!(token.next().expect(LEXER_CONSTRAINS) == "p");

    // Get problem type
    let problem_type = token.next().expect(LEXER_CONSTRAINS);
    use ProblemType::*;
    let problem_type = match problem_type {
        "sat" => Sat,
        "sate" => Sate,
        "satx" => Satx,
        "satex" => Satex,
        _ => unreachable!("{}", LEXER_CONSTRAINS),
    };

    // Reserve var capacity
    let var_capacity = token.next().expect(LEXER_CONSTRAINS);
    let var_capacity: usize = var_capacity.parse()?;
    vars.reserve(var_capacity);

    // Fill vars
    for index in 0..var_capacity {
        let var = Var::new(index);
        vars.push(var);
    }

    Ok(Some(problem_type))
}

fn clause_before_definition() -> Result<Option<ProblemType>, Box<dyn Error>> {
    Err(Box::from("Got problem clause before problem definition"))
}

fn process_sat(
    lit_path: &mut Vec<usize>,
    mut clause: Option<Clause>,
    slice: &str,
) -> Result<Option<Clause>, Box<dyn Error>> {
    clause = match slice {
        "(" => inspect_root_open(lit_path, clause)?,

        "+(" => add_clause(lit_path, clause, Or(vec![]))?,
        "*(" => add_clause(lit_path, clause, And(vec![]))?,

        ")" => inspect_root_close(lit_path, clause)?,

        "=(" => {
            return Err(Box::from(
                " '=' subclause is not allowed in problem type sat use sate or satex",
            ))
        }
        "xor(" => {
            return Err(Box::from(
                " 'xor' subclause is not allowed in problem type sat use sate or satex",
            ))
        }
        _ => unreachable!("{}", LEXER_CONSTRAINS),
    };

    Ok(clause)
}

fn process_sate(
    lit_path: &mut Vec<usize>,
    mut clause: Option<Clause>,
    slice: &str,
) -> Result<Option<Clause>, Box<dyn Error>> {
    clause = match slice {
        "(" => inspect_root_open(lit_path, clause)?,

        "+(" => add_clause(lit_path, clause, Or(vec![]))?,
        "*(" => add_clause(lit_path, clause, And(vec![]))?,
        "=(" => add_clause(lit_path, clause, Eql(vec![]))?,

        ")" => inspect_root_close(lit_path, clause)?,

        "xor(" => {
            return Err(Box::from(
                " 'xor' subclause is not allowed in problem type sat use sate or satex",
            ))
        }

        _ => unreachable!("{}", LEXER_CONSTRAINS),
    };

    Ok(clause)
}

fn process_satx(
    lit_path: &mut Vec<usize>,
    mut clause: Option<Clause>,
    slice: &str,
) -> Result<Option<Clause>, Box<dyn Error>> {
    clause = match slice {
        "(" => inspect_root_open(lit_path, clause)?,

        "+(" => add_clause(lit_path, clause, Or(vec![]))?,
        "*(" => add_clause(lit_path, clause, And(vec![]))?,
        "xor(" => add_clause(lit_path, clause, Xor(vec![]))?,

        ")" => inspect_root_close(lit_path, clause)?,

        "=(" => {
            return Err(Box::from(
                " '=' subclause is not allowed in problem type sat use sate or satex",
            ))
        }
        _ => unreachable!("{}", LEXER_CONSTRAINS),
    };

    Ok(clause)
}

fn process_satex(
    lit_path: &mut Vec<usize>,
    mut clause: Option<Clause>,
    slice: &str,
) -> Result<Option<Clause>, Box<dyn Error>> {
    clause = match slice {
        "(" => inspect_root_open(lit_path, clause)?,

        "+(" => add_clause(lit_path, clause, Or(vec![]))?,
        "*(" => add_clause(lit_path, clause, And(vec![]))?,
        "=(" => add_clause(lit_path, clause, Eql(vec![]))?,
        "xor(" => add_clause(lit_path, clause, Xor(vec![]))?,

        ")" => inspect_root_close(lit_path, clause)?,

        _ => unreachable!("{}", LEXER_CONSTRAINS),
    };

    Ok(clause)
}

use Clause::*;

fn inspect_root_open(
    lit_path: &mut Vec<usize>,
    clause: Option<Clause>,
) -> Result<Option<Clause>, Box<dyn Error>> {
    if clause.is_none() && lit_path.is_empty() {
        return Ok(clause);
    }

    Err(Box::from("Unexpected Token: ("))
}

fn add_clause(
    lit_path: &mut Vec<usize>,
    clause: Option<Clause>,
    to_add: Clause,
) -> Result<Option<Clause>, Box<dyn Error>> {
    if clause.is_none() {
        return Ok(Some(to_add));
    }

    if lit_path.is_empty() {
        let mut clause = clause.expect("Checked Above");
        let subclause = clause.inner_clauses();
        let subclause = subclause.unwrap();

        lit_path.push(subclause.len());
        subclause.push(to_add);

        return Ok(Some(clause));
    }

    let mut clause = clause.expect("Checked Above");
    let subclause = clause.inner_clauses();
    let mut subclause = subclause.expect("Due to process");

    for index in lit_path.iter() {
        let temp = &mut subclause[*index];
        let temp = temp.inner_clauses().expect("Due to process");

        subclause = temp;
    }

    lit_path.push(subclause.len());
    subclause.push(to_add);

    Ok(Some(clause))
}

fn inspect_root_close(
    lit_path: &mut Vec<usize>,
    clause: Option<Clause>,
) -> Result<Option<Clause>, Box<dyn Error>> {
    if clause.is_none() {
        return Err(Box::from("Unexpected Token: )"));
    }

    if lit_path.is_empty() {
        return Ok(clause);
    }

    lit_path.pop().expect("Checked Above");

    Ok(clause)
}
