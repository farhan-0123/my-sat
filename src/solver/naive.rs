use super::*;

use LBool::*;

pub fn naive_cnf_solver(solver: &mut Solver) -> bool {
    // Guards
    if !solver.is_cnf() {
        return false;
    }
    if solver.is_sat == False {
        return false;
    }
    if solver.is_sat == True {
        return true;
    }

    while solver.vars.increment() {
        if solver.clause.check_satisfiability(&solver.vars.values) {
            solver.is_sat = True;
            return true;
        }
    }

    solver.is_sat = False;
    false
}

impl VarDB {
    fn increment(&mut self) -> bool {
        let mut flag_break = false;

        for index in 0..self.values.len() {
            match self.values[index] {
                // Set initial condition to all true
                Undefined => {
                    self.values[index] = True;
                    flag_break = true;
                }

                // Increment in a binary fashion
                False => self.values[index] = True,
                True => {
                    self.values[index] = False;
                    flag_break = true;
                    break;
                }
            }
        }

        flag_break
    }
}

impl ClauseDB {
    fn check_satisfiability(&mut self, vars: &Vec<LBool>) -> bool {
        use Clause::*;

        if let And(subclause) = self.get_clause() {
            for clause in subclause {
                match clause {
                    Identity(index) => match vars[*index] {
                        False => return false,
                        True => continue,
                        Undefined => unreachable!("Because of fn increment"),
                    },

                    Not(index) => match vars[*index] {
                        False => continue,
                        True => return false,
                        Undefined => unreachable!("Because of fn increment"),
                    },

                    Or(lits) => {
                        let mut flag_atleast_one_true = false;

                        for lit in lits {
                            match lit {
                                Identity(index) => match vars[*index] {
                                    True => {
                                        flag_atleast_one_true = true;
                                        break;
                                    }
                                    False => continue,
                                    Undefined => unreachable!("Because of fn increment"),
                                },

                                Not(index) => match vars[*index] {
                                    False => {
                                        flag_atleast_one_true = true;
                                        break;
                                    }
                                    True => continue,
                                    Undefined => unreachable!("Because of fn increment"),
                                },

                                And(_) | Or(_) => unreachable!("Not allowed in CNF"),
                            }
                        }

                        if !flag_atleast_one_true {
                            return false;
                        }
                    }

                    And(_) => unreachable!("Not allowed in CNF"),
                }
            }
        }

        true
    }
}
