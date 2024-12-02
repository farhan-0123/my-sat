use super::*;

impl Solver {
    pub fn sat_values(&mut self) -> Option<Vec<i128>> {
        use LBool::*;

        if self.is_sat == Undefined {
            return None;
        }
        if self.is_sat == False {
            return None;
        }

        let vec_len = self.vars.values.len();
        let mut out = Vec::with_capacity(vec_len);

        for var in self.vars.iter_vars() {
            match self.vars.values[var.pos()] {
                False => out.push(-(var.name() as i128)),
                True => out.push(var.name() as i128),
                Undefined => continue,
            }
        }

        Some(out)
    }
}
