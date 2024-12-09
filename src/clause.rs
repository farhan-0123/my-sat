use crate::Var;

#[derive(Debug)]
pub enum Clause {
    Idn(Var),
    Not(Var),
    Or(Vec<Clause>),
    And(Vec<Clause>),
    Eql(Vec<Clause>),
    Xor(Vec<Clause>),
}

impl Clause {
    pub fn inner_clauses(&mut self) -> Option<&mut Vec<Clause>> {
        use Clause::*;
        match self {
            Not(_) => None,
            Idn(_) => None,
            Or(clauses) => Some(clauses),
            And(clauses) => Some(clauses),
            Eql(clauses) => Some(clauses),
            Xor(clauses) => Some(clauses),
        }
    }
}
