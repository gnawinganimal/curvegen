use crate::vec::Vec2;

pub trait Curve {
    fn get(&self, t: f32) -> Vec2;

    fn steps(&self, n: u32) -> Steps<Self> where Self: Sized {
        Steps::new(self, n)
    }
}

pub struct Steps<'a, S> where S: Curve {
    spline: &'a S,
    i: u32,
    n: u32,   
}

impl<'a, S: Curve> Steps<'a, S> {
    pub fn new(spline: &'a S, n: u32) -> Self {
        Self {
            spline,
            i: 0,
            n,
        }
    }
}

impl<'a, S: Curve> Iterator for Steps<'a, S> {
    type Item = Vec2;

    fn next(&mut self) -> Option<Vec2> {
        if self.i >= self.n {
            return None;
        }

        let t = self.i as f32 / (self.n - 1) as f32;
        self.i += 1;
        Some(self.spline.get(t))
    }
}
