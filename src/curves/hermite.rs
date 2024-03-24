
use crate::{curves::Curve, vec::Vec2};

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

#[cfg(test)]
mod tests {
    use crate::{curves::Curve, vec::Vec2};
    use super::Hermite;

    #[test]
    fn endpoints_are_correct() {
        let c = Hermite {
            p0: Vec2(0.0, 0.0),
            p1: Vec2(4.0, 1.0),
            v0: Vec2(1.0, 1.0),
            v1: Vec2(1.0, -1.0)
        };

        assert_eq!(c.get(0.0), c.p0);
        assert_eq!(c.get(1.0), c.p1);
    }
}
