use super::*;

impl Solver {
    pub fn new_var(&mut self) -> Result<Var, MySatError> {
        if self.is_locked {
            return Err(MySatError::ChangeAfterLock);
        }

        Ok(self.vars.new_var())
    }

    pub fn new_vars(&mut self, count: usize) -> Result<Vec<Var>, MySatError> {
        if self.is_locked {
            return Err(MySatError::ChangeAfterLock);
        }

        Ok(self.vars.new_vars(count))
    }

    pub fn add_clause(&mut self, clause: Clause) -> Result<&mut Self, MySatError> {
        if self.is_locked {
            return Err(MySatError::ChangeAfterLock);
        }

        self.clause.add_clause(clause);

        Ok(self)
    }

    pub fn is_cnf(&self) -> bool {
        self.clause.is_cnf()
    }
}
