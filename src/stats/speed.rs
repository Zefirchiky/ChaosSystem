use crate::stats::Stat;

pub struct Speed {
    value: isize
}

impl Stat for Speed {
    fn value(&self) -> isize {
        self.value
    }
}
