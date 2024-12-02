use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MySatError {
    // Constructor
    ChangeAfterLock,

    // Clause
    AlwaysTrue,
    IsNotCNF,

    // Vars
    CannotSetZeroAsVariableName,

    // Loading File
    EmptyFile,
    MultipleProblemDefinitions,
    ProblemClauseBeforeProblemDefinition,
}

impl Error for MySatError {}

impl Display for MySatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MySatError::*;

        match self {
            ChangeAfterLock => write!(
                f,
                "Clause or Variables are not allow to be changed after solver runs"
            ),
            AlwaysTrue => write!(f, "This clause will always be true"),
            EmptyFile => write!(f, "Found Empty file while parsing"),
            MultipleProblemDefinitions => write!(
                f,
                "Found another problem definiton after solver is initialised"
            ),
            ProblemClauseBeforeProblemDefinition => {
                write!(f, "Found problem clause before problem definition.")
            }
            CannotSetZeroAsVariableName => write!(f, "Cannot set variable name as zero"),
            IsNotCNF => write!(f, "problem clause is not cnf when solver requires it."),
        }
    }
}
