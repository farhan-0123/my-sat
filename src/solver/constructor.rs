use super::*;

impl Solver {
    pub fn new_var(&mut self) -> Var {
        self.vars.new_var()
    }

    pub fn new_vars(&mut self, count: usize) -> Vec<Var> {
        self.vars.new_vars(count)
    }

    pub fn add_clause(&mut self, clause: Clause) -> bool {
        self.clause.add_cnf_clause(clause)
    }

    pub fn is_cnf(&self) -> bool {
        self.clause.is_cnf()
    }
}
