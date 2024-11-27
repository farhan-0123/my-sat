use crate::*;

mod constructor;
mod naive;
mod output;
mod solve;

pub struct Solver {
    is_sat: LBool,
    vars: VarDB,
    clause: ClauseDB,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            is_sat: LBool::Undefined,
            vars: VarDB::new(),
            clause: ClauseDB::new(),
        }
    }

    pub fn with_capacity(vars: usize, clause: usize) -> Self {
        Self {
            is_sat: LBool::Undefined,
            vars: VarDB::with_capacity(vars),
            clause: ClauseDB::with_capacity(clause),
        }
    }
}
