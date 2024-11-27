#[derive(Debug)]
pub enum MySatError {
    ChangeAfterLock,
    AlwaysTrue,
}