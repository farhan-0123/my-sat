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

    // To remove clutter
    let values = &mut solver.vars.values;
    let clause = &solver.clause;

    // Implementation
    while increment(values) {
        if check_satisfiability(clause, values) {
            solver.is_sat = True;
            return true;
        }
    }

    // If we checked all combinations then this is not satisfiable.
    solver.is_sat = False;
    false
}

fn increment(values: &mut Vec<LBool>) -> bool {
    let mut flag_break = false;

    for index in 0..values.len() {
        match values[index] {
            // Set initial condition to all True
            Undefined => {
                values[index] = True;
                flag_break = true;
            }

            // Increment in a binary fashion
            False => values[index] = True,
            True => {
                values[index] = False;
                flag_break = true;
                break;
            }
        }
    }

    // If the for loop never encounters True(all the values are False)
    // then we have checked all the values.
    // In other words, false if all combinations checked
    flag_break
}

fn check_satisfiability(clause: &ClauseDB, values: &Vec<LBool>) -> bool {
    use Clause::*;

    if let And(subclause) = clause.get_clause() {
        for clause in subclause {
            match clause {
                Identity(var) => match values[var.pos()] {
                    False => return false,
                    True => continue,
                    Undefined => unreachable!("Because of fn increment"),
                },

                Not(var) => match values[var.pos()] {
                    False => continue,
                    True => return false,
                    Undefined => unreachable!("Because of fn increment"),
                },

                Or(lits) => {
                    let mut flag_atleast_one_true = false;

                    for lit in lits {
                        match lit {
                            Identity(var) => match values[var.pos()] {
                                True => {
                                    flag_atleast_one_true = true;
                                    break;
                                }
                                False => continue,
                                Undefined => unreachable!("Because of fn increment"),
                            },

                            Not(var) => match values[var.pos()] {
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
