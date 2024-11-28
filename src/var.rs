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
    pub fn new_var(&mut self) -> Var {
        assert!(self.values.len() == self.vars.len());

        let value = Var {
            name: self.vars.len() + 1,
            pos: self.vars.len(),
        };

        self.vars.push(value);
        self.values.push(LBool::Undefined);

        value
    }

    pub fn new_vars(&mut self, count: usize) -> Vec<Var> {
        let mut vars = Vec::with_capacity(count);

        for _ in 0..count {
            vars.push(self.new_var());
        }

        vars
    }

    pub fn get_var(&self, index: usize) -> Var {
        self.vars[index]
    }
}
