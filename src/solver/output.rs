use super::*;

impl Solver {
    pub fn sat_values(&mut self) -> Vec<i128> {
        self.vars
        .get_values()
        .iter()
        .map(|inp| match_lbool(inp)).collect()
    }
}

fn match_lbool(inp: &LBool) -> i128 {
    match inp {
        LBool::False => return -1,
        LBool::True => 1,
        LBool::Undefined => 0,
    }
}