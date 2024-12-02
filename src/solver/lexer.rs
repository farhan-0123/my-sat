use std::error::Error;
use std::fs;

use crate::*;

use logos::*;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum DimacsCnf {
    #[regex("c.*\n")]
    Comment,
    #[regex("p +cnf +[0-9]+ +[0-9]+ *\n")]
    Problem,
    #[regex(" *([+-]{0, 1}[0-9]+ +)+0\n")]
    Clause,
    #[regex("%\n0\n\n")]
    End,
}

impl Solver {
    pub fn open_cnf(path: &str) -> Result<Self, Box<dyn Error>> {
        let source = fs::read_to_string(path)?;
        let mut lexer = DimacsCnf::lexer(source.as_str());

        let mut solver: Option<Solver> = None;

        use DimacsCnf::*;
        loop {
            match lexer.next() {
                Some(Ok(token)) if token == Comment => continue,

                Some(Ok(token)) if token == Problem => {
                    solver = process_problem(&solver, lexer.slice())?
                }
                Some(Ok(token)) if token == Clause => process_clause(&mut solver, lexer.slice())?,
                Some(Ok(token)) if token == End => process_end(&mut solver)?,

                Some(Err(_)) => panic!("Unexpeted Token, got: {}", lexer.slice()),

                None => break,

                Some(Ok(token)) => unreachable!(
                    "DimacsCnf enum not implemented in match expression: {:?}",
                    token
                ),
            }
        }
        if let Some(solver) = solver {
            Ok(solver)
        } else {
            Err(Box::new(MySatError::EmptyFile))
        }
    }
}

const LEXER_CONSTRAINS: &str = "Should not Panic due to lexer constrains";

fn process_problem(solver: &Option<Solver>, slice: &str) -> Result<Option<Solver>, Box<dyn Error>> {
    if solver.is_some() {
        return Err(Box::new(MySatError::MultipleProblemDefinitions));
    }

    let mut token = slice.split_whitespace();

    // Ignoring first two values
    assert!(token.next().expect(LEXER_CONSTRAINS) == "p");
    assert!(token.next().expect(LEXER_CONSTRAINS) == "cnf");

    // Init Vars
    let vars = token.next().expect(LEXER_CONSTRAINS);
    let vars: usize = vars.parse()?;

    // Init Clause
    let clause = token.next().expect(LEXER_CONSTRAINS);
    let clause: usize = clause.parse()?;

    // Init & Return Structs
    Ok(Some(Solver::with_capacity(vars, clause)))
}

fn process_clause(solver: &mut Option<Solver>, slice: &str) -> Result<(), Box<dyn Error>> {
    if let Some(solver) = solver {
        let token = slice.split_whitespace();
        let mut clause: Vec<Clause> = vec![];

        use Clause::*;

        for item in token {
            let item: isize = item.parse()?;
            if item == 0 {
                break;
            }

            if item < 0 {
                let var = solver.get_or_new_var((-item) as usize)?;
                clause.push(Not(var));
            }

            if item > 0 {
                let var = solver.get_or_new_var(item as usize)?;
                clause.push(Identity(var));
            }
        }

        solver.add_clause(Or(clause))?;

        Ok(())
    } else {
        return Err(Box::new(MySatError::ProblemClauseBeforeProblemDefinition));
    }
}

fn process_end(solver: &mut Option<Solver>) -> Result<(), MySatError> {
    if let Some(solver) = solver {
        if !solver.is_cnf() {
            return Err(MySatError::IsNotCNF);
        }

        Ok(())
    } else {
        return Err(MySatError::EmptyFile);
    }
}
