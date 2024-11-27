use super::*;

use naive::naive_cnf_solver;

impl Solver {
    pub fn solve(&mut self) -> bool {
        naive_cnf_solver(self)
    }
}
