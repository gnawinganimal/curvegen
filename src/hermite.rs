
use crate::{spline::Spline, vec::Vec2};

pub struct Hermite { 
    pub p0: Vec2,
    pub p1: Vec2,
    pub v0: Vec2,
    pub v1: Vec2,
}

macro_rules! sum {
    ($h:expr) => ($h);
    ($h:expr, $($t:expr),* $(,)*) =>
        ($h + sum!($($t),*));
}

impl Spline for Hermite {
    fn p0(&self) -> Vec2 {
        self.p0
    }

    fn p1(&self) -> Vec2 {
        self.p1
    }

    fn get(&self, t: f32) -> Option<Vec2> {
        if t < 0.0 || t > 1.0 {
            return None;
        }

        Some(sum! {
            (2. * self.p0 + self.v0 - 2. * self.p1 + self.v1) * t.powi(3),
            (-3. * self.p0 + 3. * self.p1 - 2. * self.v0 - self.v1) * t.powi(2),
            self.v0 * t,
            self.p0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{spline::Spline, vec::Vec2};
    use super::Hermite;

    #[test]
    fn get_t_0() {
        let c = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        assert_eq!(c.get(0.0), Some(Vec2(0.0, 0.0)))
    }

    #[test]
    fn get_t_1() {
        let c = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        assert_eq!(c.get(1.0), Some(Vec2(4.0, 1.0)))
    }

    #[test]
    fn get_t_lt_0() {
        let c = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        assert_eq!(c.get(-0.001), None)
    }

    #[test]
    fn get_t_gt_1() {
        let c = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        assert_eq!(c.get(1.001), None)
    }

    #[test]
    fn steps_has_correct_lengths() {
        let c = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        let steps: Vec<Vec2> = c.steps(10).collect();

        assert_eq!(steps.len(), 10);
    }

    #[test]
    fn steps_has_correct_endpoints() {
        let c = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        let steps: Vec<Vec2> = c.steps(10).collect();

        assert_eq!(steps[0], Vec2(0.0, 0.0));
        assert_eq!(steps[9], Vec2(4.0, 1.0));
    }
}
