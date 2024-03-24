use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec2(pub f32, pub f32);

impl Vec2 {
    pub fn norm(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2(-self.0, -self.1)
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, n: f32) -> Self::Output {
        Vec2(self.0 + n, self.1 + n)
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;

    fn sub(self, n: f32) -> Self::Output {
        Vec2(self.0 - n, self.1 - n)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, n: f32) -> Self::Output {
        Vec2(self.0 * n, self.1 * n)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, n: f32) -> Self::Output {
        Vec2(self.0 / n, self.1 / n)
    }
}

impl Add<Vec2> for f32 {
    type Output = Vec2;

    fn add(self, v: Vec2) -> Self::Output {
        Vec2(self + v.0, self + v.1)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Self::Output {
        Vec2(self * v.0, self * v.1)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, v: Vec2) -> Self::Output {
        Self(self.0 + v.0, self.1 + v.1)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, v: Vec2) -> Self::Output {
        Self(self.0 - v.0, self.1 - v.1)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::vec::Vec2;

    #[test]
    fn norm_of_vec2() {
        assert_eq!(Vec2(3.0, 4.0).norm(), 5.0)
    }

    #[test]
    fn add_f32_to_vec2() {
        assert_eq!(Vec2(1.0, 4.0) + 3.0, Vec2(4.0, 7.0))
    }

    #[test]
    fn sub_f32_from_vec2() {
        assert_eq!(Vec2(6.0, 4.0) - 3.0, Vec2(3.0, 1.0))
    }

    #[test]
    fn mul_vec2_by_f32() {
        assert_eq!(Vec2(3.0, 2.0) * 2.0, Vec2(6.0, 4.0))
    }

    #[test]
    fn div_vec2_by_f32() {
        assert_eq!(Vec2(8.0, 6.0) / 2.0, Vec2(4.0, 3.0))
    }

    #[test]
    fn add_vec2_to_vec2() {
        assert_eq!(Vec2(1.0, 4.0) + Vec2(6.0, 2.0), Vec2(7.0, 6.0))
    }

    #[test]
    fn sub_vec2_from_vec2() {
        assert_eq!(Vec2(6.0, 4.0) - Vec2(4.0, 2.0), Vec2(2.0, 2.0))
    }
}
