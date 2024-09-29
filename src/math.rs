use std::ops;

pub struct Vector2(pub i32, pub i32);

impl Default for Vector2 {
    fn default() -> Self {
        return Vector2 (0, 0);
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        return Vector2(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl ops::Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: i32) -> Self::Output {
        return Vector2(self.0 * rhs, self.1 * rhs);
    }
}

impl ops::MulAssign<i32> for Vector2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl ops::Div<i32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: i32) -> Self::Output {
        return Vector2(self.0 / rhs, self.1 * rhs);
    }
}   