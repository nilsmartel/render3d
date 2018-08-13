

use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vector3(f32, f32, f32);

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3(x, y, z)
    }
}

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

#[derive(Debug, Copy, Clone)]
pub struct Vector2(f32, f32);

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2(x, y)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector2(self.0 * scalar, self.1 * scalar)
    }
}

pub struct Matrix3x3(Vector3, Vector3, Vector3);

impl Matrix3x3 {
    pub fn identity() -> Matrix3x3 {
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

pub struct Matrix2x2(Vector2, Vector2);

impl Matrix2x2 {
    pub fn identity() -> Matrix2x2 {
        Matrix2x2(
            Vector2(1.0, 0.0),
            Vector2(0.0, 1.0)
        )
    }
}

impl Mul<Vector2> for Matrix2x2 {
    type Output = Vector2;

    fn mul(self, v: Vector2) -> Self::Output {
        Vector2(
            v.0 * (self.0).0,
            v.1 * (self.1).1
        ) +
        Vector2(
            v.0 * (self.0).0,
            v.1 * (self.1).1
        )
    }
}