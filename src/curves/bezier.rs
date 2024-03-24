use crate::{curves::Curve, vec::Vec2};

use super::hermite::Hermite;

pub struct Bezier { 
    pub p0: Vec2,
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
}

impl Curve for Bezier {
    fn get(&self, t: f32) -> Vec2 {
        (1.0 - t).powi(3) * self.p0 +
        3.0 * (1.0 - t).powi(2) * t * self.p1 +
        3.0 * (1.0 - t) * t.powi(2) * self.p2 +
        t.powi(3) * self.p3
    }
}

impl From<&Hermite> for Bezier {
    fn from(c: &Hermite) -> Self {
        Self {
            p0: c.p0,
            p1: c.p0 + (c.v0 / 3.),
            p2: c.p1 - (c.v1 / 3.),
            p3: c.p1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{curves::{hermite::Hermite, Curve}, vec::Vec2};
    use super::Bezier;

    use approx::assert_ulps_eq;

    #[test]
    pub fn endpoints_are_correct() {
        let c = Bezier {
            p0: Vec2(1.0, 1.0),
            p1: Vec2(5.0, 4.0),
            p2: Vec2(3.0, 5.5),
            p3: Vec2(4.0, 1.0),
        };

        assert_eq!(c.get(0.0), c.p0);
        assert_eq!(c.get(1.0), c.p3);
    }

    #[test]
    pub fn converts_endpoints_from_hermite() {
        let h = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        let b = Bezier::from(&h);
        
        assert_eq!(h.get(0.), b.get(0.));
        assert_eq!(h.get(1.), b.get(1.)); 
    }

    #[test]
    pub fn converts_hermite_to_bezier() {
        let h = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        let b = Bezier::from(&h);

        assert_ulps_eq!(h.get(0.0), b.get(0.0));
        assert_ulps_eq!(h.get(0.2), b.get(0.2));
        assert_ulps_eq!(h.get(0.4), b.get(0.4));
        assert_ulps_eq!(h.get(0.6), b.get(0.6));
        assert_ulps_eq!(h.get(0.8), b.get(0.8));
        assert_ulps_eq!(h.get(1.0), b.get(1.0));
    }
}
