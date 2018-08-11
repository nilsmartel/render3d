
use std::ops::{Add, Mul};

fn main() {
    let v = Vector3(1.0, 2.0, 3.0);

    println!("{:?}", Matrix3x3::identity() * v);
}

#[derive(Debug)]
struct Vector3(f32, f32, f32);

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector3(
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar
        )
    }
}

struct Matrix3x3(Vector3, Vector3, Vector3);

impl Matrix3x3 {
    fn identity() -> Matrix3x3 {
        Matrix3x3(
            Vector3(1.0, 0.0, 0.0),
            Vector3(0.0, 1.0, 0.0),
            Vector3(0.0, 0.0, 1.0)
        )
    }
}

impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Self::Output {
        Vector3(
            v.0 * (self.0).0,
            v.1 * (self.1).1,
            v.2 * (self.2).2
        ) + 
        Vector3(
            v.0 * (self.0).0,
            v.1 * (self.1).1,
            v.2 * (self.2).2
        ) + 
        Vector3(
            v.0 * (self.0).0,
            v.1 * (self.1).1,
            v.2 * (self.2).2
        )
    }
}
