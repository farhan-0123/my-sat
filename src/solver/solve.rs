use super::*;

// use naive::naive_cnf_solver;
use dpll::dpll_cnf_solver;

impl Solver {
    pub fn solve(&mut self) -> bool {
        self.is_locked = true;

        // naive_cnf_solver(self)
        dpll_cnf_solver(self)
    }
}
