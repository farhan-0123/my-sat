#[derive(Debug)]
pub enum Portfolio {
    BruteForce,
    LocalSearch,
    DFS,  // Depth First Search
    DAC,  // Divide and conqure
    DPLL, // Davis Putnam Logemann Loveland
    CDCL, // Conflict Driven Clause Learning
}

#[derive(Debug, PartialEq)]
pub enum ProblemType {
    #[expect(unused)]
    Cnf,
    Sat,
    Satx,
    Sate,
    Satex,
}
