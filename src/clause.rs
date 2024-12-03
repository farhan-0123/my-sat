use crate::*;

#[derive(Debug)]
pub enum Clause {
    Identity(Var),
    Not(Var),
    Or(Vec<Clause>),
    And(Vec<Clause>),
}

#[derive(Debug)]
pub struct ClauseDB {
    clause: Clause,
}

impl ClauseDB {
    pub fn new() -> Self {
        Self {
            clause: Clause::And(vec![]),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            clause: Clause::And(Vec::with_capacity(capacity)),
        }
    }
}

impl ClauseDB {
    pub fn get_clause(&self) -> &Clause {
        &self.clause
    }

    pub fn add_clause(&mut self, clause: Clause) -> bool {
        use Clause::*;

        if let And(vec) = &mut self.clause {
            vec.push(clause);
        }

        true
    }
}

impl ClauseDB {
    pub fn is_cnf(&self) -> bool {
        use Clause::*;

        if let And(subclauses) = &self.clause {
            for clause in subclauses {
                match clause {
                    Identity(_) | Not(_) => continue,

                    Or(vars) => {
                        for var in vars {
                            match var {
                                Identity(_) | Not(_) => continue,
                                And(_) | Or(_) => return false,
                            }
                        }
                    }

                    And(_) => return false,
                }
            }
        } else {
            return false;
        }

        true
    }
}
