use std::error::Error;
use std::fmt::Display;

#[expect(unused)]
#[derive(Debug)]
pub struct SolverError {
    code: usize,
    message: String,
}

impl Error for SolverError {}

#[expect(unused)]
impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
