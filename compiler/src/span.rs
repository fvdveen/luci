mod symbol;

pub use symbol::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Span {
    lo: u32,
    hi: u32,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Span {
        Span {
            lo: lo as u32,
            hi: hi as u32,
        }
    }
}
