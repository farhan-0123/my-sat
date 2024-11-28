use std::fs;

use crate::*;

use logos::*;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum DimacsCnf {
    #[regex("c.*\n")]
    Comment,
    #[regex("p +cnf +[0-9]+ +[0-9]+ *\n")]
    Problem,
    #[regex(" *([+-]{0, 1}[0-9]+ +)+0\n")]
    Or,
    #[regex("%\n0\n\n")]
    End,
}

impl Solver {
    pub fn open_cnf() -> Self {
        todo!()
    }
}

pub fn test_lexer() {
    let path = "./cnf_test_suit/uf20-91/uf20-01.cnf";
    let result = fs::read_to_string(path).unwrap();

    let mut my_lexer = DimacsCnf::lexer(result.as_str());

    let mut count = 0u8;

    use DimacsCnf::*;
    loop {
        match my_lexer.next() {
            Some(Ok(token)) if token == Or => {
                count += 1;
                println!("{:02?} {:?} {:>15}", count, token, my_lexer.slice());
            }
            Some(Ok(token)) => println!("{:?} {:?}", token, my_lexer.slice()),
            Some(Err(_)) => println!("Unexpected: {:?}", my_lexer.slice()),
            None => break,
        }
    }
}
