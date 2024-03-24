use crate::{vec::Vec2, curves::steps::Steps};

pub trait Curve {
    fn get(&self, t: f32) -> Vec2;

    fn steps(&self, n: u32) -> Steps<Self> where Self: Sized {
        Steps::new(self, n)
    }
}
