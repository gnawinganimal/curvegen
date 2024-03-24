use crate::{curves::Curve, vec::Vec2};

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

#[cfg(test)]
mod tests {
    use crate::{curves::Curve, vec::Vec2};
    use super::Bezier;

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
}
