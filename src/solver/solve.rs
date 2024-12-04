use super::*;

use dpll::dpll_cnf_solver;
// use naive::naive_cnf_solver;

use LBool::*;

impl Solver {
    pub fn solve(&mut self) -> bool {
        self.is_locked = true;

        // Guards
        if !self.is_cnf() {
            return false;
        }
        if self.is_sat == False {
            return false;
        }
        if self.is_sat == True {
            return true;
        }

        // naive_cnf_solver(self)
        dpll_cnf_solver(self)
    }
}
