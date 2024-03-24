use crate::vec::Vec2;


pub trait Spline {
    fn get(&self, t: f32) -> Option<Vec2>;

    fn p0(&self) -> Vec2;

    fn p1(&self) -> Vec2;

    fn steps(&self, n: u32) -> Steps<Self> where Self: Sized {
        Steps::new(self, n)
    }
}

pub struct Steps<'a, S> where S: Spline {
    spline: &'a S,
    i: u32,
    n: u32,   
}

impl<'a, S: Spline> Steps<'a, S> {
    pub fn new(spline: &'a S, n: u32) -> Self {
        Self {
            spline,
            i: 0,
            n,
        }
    }

    pub fn arc_len(mut self) -> f32 {
        let mut ds = 0.0;
        let mut v0 = self.next().unwrap();

        while let Some(v1) = self.next() {
            ds += (v1 - v0).norm();
            v0 = v1;
        };

        ds
    }
}

impl<'a, S: Spline> Iterator for Steps<'a, S> {
    type Item = Vec2;

    fn next(&mut self) -> Option<Vec2> {
        if self.i >= self.n {
            return None;
        }

        let t = self.i as f32 / (self.n - 1) as f32;
        self.i += 1;
        self.spline.get(t)
    }
}
