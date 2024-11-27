use super::*;

impl Solver {
    pub fn sat_values(&mut self) -> Vec<i128> {
        let vec_len = self.vars.values.len();
        let mut out = Vec::with_capacity(vec_len);

        use LBool::*;
        for index in 0..vec_len {
            match self.vars.values[index] {
                False => out.push(-(self.vars.get_var(index).name() as i128)),
                True => out.push(self.vars.get_var(index).name() as i128),
                Undefined => out.push(0),
            }
        }

        out
    }
}
