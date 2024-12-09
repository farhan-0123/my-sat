mod cnf;
mod sat;

use std::error::Error;
use std::path::PathBuf;

use crate::Solver;

use cnf::open_dimacs_cnf;
use sat::open_dimacs_sat;

pub const LEXER_CONSTRAINS: &str = "Should not Panic due to lexer constrains";

impl Solver {
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        let path: PathBuf = path.try_into()?;
        match path.extension() {
            Some(ext) if ext == "cnf" => open_dimacs_cnf(path),
            Some(ext) if ext == "sat" => open_dimacs_sat(path),

            Some(ext) => {
                return Err(Box::from(format!(
                    "Found unknown file extention, got: {:?}",
                    ext
                )))
            }

            None => return Err(Box::from("No File extention Found")),
        }
    }
}
