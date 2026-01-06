mod speed;
mod strength;
mod default_stats;

pub use speed::Speed;
pub use strength::Strength;
pub use default_stats::Stats;

pub trait Stat {
    fn value(&self) -> isize;
}