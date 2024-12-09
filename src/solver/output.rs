use std::error::Error;

use crate::Solver;

#[expect(unused)]
impl Solver {
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
