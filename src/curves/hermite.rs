
use crate::curves::Curve;
use glam::Vec2;

use super::bezier::Bezier;

pub struct Hermite { 
    pub p0: Vec2,
    pub p1: Vec2,
    pub v0: Vec2,
    pub v1: Vec2,
}

impl Curve for Hermite {
    fn get(&self, t: f32) -> Vec2 {
        (2. * self.p0 + self.v0 - 2. * self.p1 + self.v1) * t.powi(3) +
        (-3. * self.p0 + 3. * self.p1 - 2. * self.v0 - self.v1) * t.powi(2) +
        self.v0 * t +
        self.p0
    }
}

impl From<&Bezier> for Hermite {
    fn from(b: &Bezier) -> Self {
        Self {
            p0: b.p0,
            p1: b.p3,
            v0: (b.p1 - b.p0) * 3.,
            v1: (b.p2 - b.p3) * -3.,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::curves::{bezier::Bezier, Curve};
    use super::Hermite;
    use glam::vec2;

    use approx::assert_ulps_eq;

    #[test]
    fn endpoints_are_correct() {
        let c = Hermite {
            p0: vec2(0.0, 0.0),
            p1: vec2(4.0, 1.0),
            v0: vec2(1.0, 1.0),
            v1: vec2(1.0, -1.0)
        };

        assert_eq!(c.get(0.0), c.p0);
        assert_eq!(c.get(1.0), c.p1);
    }

    #[test]
    pub fn converts_endpoints_from_hermite() {
        let b = Bezier {
            p0: vec2(1.0, 1.0),
            p1: vec2(5.0, 4.0),
            p2: vec2(3.0, 5.5),
            p3: vec2(4.0, 1.0),
        };

        let h = Hermite::from(&b);
        
        assert_eq!(h.get(0.), b.get(0.));
        assert_eq!(h.get(1.), b.get(1.)); 
    }

    #[test]
    pub fn converts_hermite_to_bezier() {
        let b = Bezier {
            p0: vec2(1.0, 1.0),
            p1: vec2(5.0, 4.0),
            p2: vec2(3.0, 5.5),
            p3: vec2(4.0, 1.0),
        };

        let h = Hermite::from(&b);

        assert_ulps_eq!(b.get(0.0), h.get(0.0));
        assert_ulps_eq!(b.get(0.2), h.get(0.2));
        assert_ulps_eq!(b.get(0.4), h.get(0.4));
        assert_ulps_eq!(b.get(0.6), h.get(0.6));
        assert_ulps_eq!(b.get(0.8), h.get(0.8));
        assert_ulps_eq!(b.get(1.0), h.get(1.0));
    }
}
