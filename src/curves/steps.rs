use crate::{vec::Vec2, curves::Curve};

pub struct Steps<'a, C: Curve> {
    curve: &'a C,
    i: u32,
    n: u32,   
}

impl<'a, C: Curve> Steps<'a, C> {
    pub fn new(curve: &'a C, n: u32) -> Self {
        Self {
            curve,
            i: 0,
            n,
        }
    }
}

impl<'a, C: Curve> Iterator for Steps<'a, C> {
    type Item = Vec2;

    fn next(&mut self) -> Option<Vec2> {
        if self.i >= self.n {
            return None;
        }

        let t = self.i as f32 / (self.n - 1) as f32;
        self.i += 1;
        Some(self.curve.get(t))
    }
}

#[cfg(test)]
mod tests {
    use crate::{curves::{bezier::Bezier, Curve}, vec::Vec2};

    #[test]
    fn steps_produces_correct_number_of_points() {
        let c = Bezier {
            p0: Vec2(1.0, 1.0),
            p1: Vec2(5.0, 4.0),
            p2: Vec2(3.0, 5.5),
            p3: Vec2(4.0, 1.0),
        };

        let p: Vec<Vec2> = c.steps(10).collect();
        assert_eq!(p.len(), 10);
    }

    #[test]
    fn steps_endpoints_are_correct() {
        let c = Bezier {
            p0: Vec2(1.0, 1.0),
            p1: Vec2(5.0, 4.0),
            p2: Vec2(3.0, 5.5),
            p3: Vec2(4.0, 1.0),
        };

        let p: Vec<Vec2> = c.steps(10).collect();
        assert_eq!(p[0], Vec2(1.0, 1.0));
        assert_eq!(p[9], Vec2(4.0, 1.0));
    }
}
