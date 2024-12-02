use std::ops::Range;

use super::*;

impl Solver {
    pub fn get_or_new_var(&mut self, name: usize) -> Result<Var, MySatError> {
        if self.is_locked {
            return Err(MySatError::ChangeAfterLock);
        }

        Ok(self.vars.get_or_new_var(name))
    }

    pub fn get_or_new_vars(&mut self, range: Range<usize>) -> Result<Vec<Var>, MySatError> {
        if self.is_locked {
            return Err(MySatError::ChangeAfterLock);
        }
        if range.start == 0 {
            return Err(MySatError::CannotSetZeroAsVariableName);
        }
        Ok(self.vars.get_or_new_vars(range))
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
