mod input;
mod output;
mod solve;

use crate::Clause;
use crate::Portfolio;
use crate::Var;

#[derive(Debug)]
pub struct Solver {
    portfolio: Portfolio,
    clause: Clause,
    vars: Vec<Var>,
    values: Option<Vec<i64>>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            portfolio: Portfolio::BruteForce,
            clause: Clause::And(vec![]),
            vars: vec![],
            values: None,
        }
    }

    pub fn set_portfolio(&mut self, portfolio: Portfolio) {
        self.portfolio = portfolio
    }

    pub fn new_var(&mut self) -> Var {
        let var = Var::new(self.vars.len());
        self.vars.push(var);
        var
    }

    pub fn new_vars(&mut self, count: usize) -> Vec<Var> {
        self.vars.reserve(count);

        let mut vec = Vec::with_capacity(count);

        for _ in 0..count {
            vec.push(self.new_var());
        }

        vec
    }

    pub fn set_clause(&mut self, clause: Clause) {
        self.clause = clause
    }

    pub fn sat_values(&self) -> Option<Vec<i64>> {
        self.values.clone()
    }
}
