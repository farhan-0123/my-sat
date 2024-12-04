use super::*;

use Clause::*;
use LBool::*;

pub fn dpll_cnf_solver(solver: &mut Solver) -> bool {
    let values = &mut solver.vars.values;
    let clause = solver.clause.get_clause();

    let mut index: usize = 0;

    loop {
        solver.search_count += 1;

        match values[index] {
            Undefined => {
                values[index] = True;
                index += 1;
            }
            True => {
                values[index] = False;
                index += 1;
            }
            False => {
                values[index] = Undefined;
                if index == 0 {
                    break;
                }
                index -= 1;
            }
        }

        match check_satisfiability(values, clause) {
            Undefined => continue,

            True => {
                solver.is_sat = True;
                return true;
            }

            False => index -= 1,
        }
    }

    false
}

// Undefined -> No Conflict, Not All required variables are set
// True      -> No Conflict,     All required variables are set
// False     ->    Conflict, (Doesn't matter if all variables are set or not)
fn check_satisfiability(values: &Vec<LBool>, clause: &Clause) -> LBool {
    if let And(subclauses) = clause {
        let mut atleast_one_undefined_subclause = false;

        for subclause in subclauses {
            match subclause {
                Identity(var) if values[var.pos()] == True => continue,
                Identity(var) if values[var.pos()] == False => return False,
                Identity(var) if values[var.pos()] == Undefined => {
                    atleast_one_undefined_subclause = true
                }

                Not(var) if values[var.pos()] == False => continue,
                Not(var) if values[var.pos()] == True => return False,
                Not(var) if values[var.pos()] == Undefined => {
                    atleast_one_undefined_subclause = true
                }

                Or(lits) => {
                    let mut atlease_one_true = false;
                    let mut atleast_one_undefined = false;

                    for lit in lits {
                        match lit {
                            Identity(var) if values[var.pos()] == True => {
                                atlease_one_true = true;
                                break;
                            }
                            Identity(var) if values[var.pos()] == False => continue,
                            Identity(var) if values[var.pos()] == Undefined => {
                                atleast_one_undefined = true
                            }

                            Not(var) if values[var.pos()] == False => {
                                atlease_one_true = true;
                                break;
                            }
                            Not(var) if values[var.pos()] == True => continue,
                            Not(var) if values[var.pos()] == Undefined => {
                                atleast_one_undefined = true
                            }

                            Identity(_) | Not(_) => unreachable!("Checked all branches"),
                            And(_) | Or(_) => unreachable!("Due to cnf rules"),
                        }
                    }

                    if atlease_one_true {
                        continue;
                    } else if !atlease_one_true && atleast_one_undefined {
                        atleast_one_undefined_subclause = true;
                    } else if !atlease_one_true && !atleast_one_undefined {
                        return False;
                    } else {
                        unreachable!("All varients checked")
                    }
                }

                Identity(_) | Not(_) => unreachable!("Checked all branches"),
                And(_) => unreachable!("Due to cnf rules"),
            }
        }

        if atleast_one_undefined_subclause {
            return Undefined;
        }

        return True;
    }

    unreachable!("Due to cnf rules")
}
