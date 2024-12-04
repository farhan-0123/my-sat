use super::*;

use LBool::*;

#[expect(unused)]
pub fn naive_cnf_solver(solver: &mut Solver) -> bool {
    // To remove clutter
    let values = &mut solver.vars.values;
    let clause = &solver.clause;

    // Implementation
    while increment(values) {
        solver.search_count += 1;

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
                Identity(var) if values[var.pos()] == True => continue,
                Identity(var) if values[var.pos()] == False => return false,

                Not(var) if values[var.pos()] == False => continue,
                Not(var) if values[var.pos()] == True => return false,

                Or(lits) => {
                    let mut atleast_one_true = false;

                    for lit in lits {
                        match lit {
                            Identity(var) if values[var.pos()] == False => continue,
                            Identity(var) if values[var.pos()] == True => {
                                atleast_one_true = true;
                                break;
                            }

                            Not(var) if values[var.pos()] == True => continue,
                            Not(var) if values[var.pos()] == False => {
                                atleast_one_true = true;
                                break;
                            }

                            Identity(_) | Not(_) => unreachable!("All case tested"),
                            And(_) | Or(_) => unreachable!("Not allowed in CNF"),
                        }
                    }

                    if !atleast_one_true {
                        return false;
                    }
                }

                Identity(_) | Not(_) => unreachable!("All case tested"),
                And(_) => unreachable!("Not allowed in CNF"),
            }
        }
    }

    true
}
