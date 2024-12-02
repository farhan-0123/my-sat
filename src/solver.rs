use crate::*;

mod constructor;
mod dpll;
mod lexer;
mod naive;
mod output;
mod solve;

#[derive(Debug)]
pub struct Solver {
    is_locked: bool,
    is_sat: LBool,
    vars: VarDB,
    clause: ClauseDB,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            is_locked: false,
            is_sat: LBool::Undefined,
            vars: VarDB::new(),
            clause: ClauseDB::new(),
        }
    }

    pub fn with_capacity(vars: usize, clause: usize) -> Self {
        Self {
            is_locked: false,
            is_sat: LBool::Undefined,
            vars: VarDB::with_capacity(vars),
            clause: ClauseDB::with_capacity(clause),
        }
    }
}
