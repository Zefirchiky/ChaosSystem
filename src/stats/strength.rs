use crate::stats::Stat;

pub struct Strength {
    value: isize,
}

impl Stat for Strength {
    fn value(&self) -> isize {
        self.value
    }
}