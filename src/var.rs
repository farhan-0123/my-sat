use std::ops::Range;
use std::slice::Iter;

use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Var {
    name: usize,
    pos: usize,
}

impl Var {
    pub fn name(&self) -> usize {
        self.name
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
pub struct VarDB {
    vars: Vec<Var>,
    pub values: Vec<LBool>,
}

impl VarDB {
    pub fn new() -> Self {
        Self {
            vars: vec![],
            values: vec![],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vars: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }
}

impl VarDB {
    pub fn get_or_new_vars(&mut self, range: Range<usize>) -> Vec<Var> {
        let mut out: Vec<Var> = Vec::with_capacity(range.len());

        for name in range {
            out.push(self.get_or_new_var(name));
        }

        out
    }

    pub fn get_or_new_var(&mut self, name: usize) -> Var {
        for var in self.iter_vars() {
            if var.name() == name {
                return *var;
            }
        }

        let var = Var {
            pos: self.vars.len(),
            name,
        };
        self.vars.push(var);
        self.values.push(LBool::Undefined);

        var
    }
}

impl VarDB {
    pub fn iter_vars(&self) -> Iter<'_, Var> {
        self.vars.iter()
    }
}
