#[derive(Debug, Clone, Copy)]
pub struct Var {
    pos: usize,
}

impl Var {
    pub fn new(pos: usize) -> Self {
        Var { pos }
    }
    pub fn pos(&self) -> usize {
        self.pos
    }
}
