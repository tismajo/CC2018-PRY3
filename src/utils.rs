use raylib::prelude::*;
use std::f32::consts::PI;

pub trait Vector3Ext {
    fn zero() -> Vector3;
    fn normalized(&self) -> Vector3;
    fn cross(&self, other: Vector3) -> Vector3;
    fn dot(&self, other: Vector3) -> f32;
    fn transform(&self, matrix: &Matrix) -> Vector3;
    fn scale_by(&self, scalar: f32) -> Vector3;
}

impl Vector3Ext for Vector3 {
    fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    fn normalized(&self) -> Vector3 {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length > 0.0 {
            Vector3::new(self.x / length, self.y / length, self.z / length)
        } else {
            *self
        }
    }

    fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn dot(&self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn transform(&self, matrix: &Matrix) -> Vector3 {
        Vector3::new(
            self.x * matrix.m0 + self.y * matrix.m4 + self.z * matrix.m8 + matrix.m12,
            self.x * matrix.m1 + self.y * matrix.m5 + self.z * matrix.m9 + matrix.m13,
            self.x * matrix.m2 + self.y * matrix.m6 + self.z * matrix.m10 + matrix.m14,
        )
    }

    fn scale_by(&self, scalar: f32) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

pub trait MatrixExt {
    fn rotate_y(angle: f32) -> Matrix;
    fn rotate(axis: Vector3, angle: f32) -> Matrix;
}

impl MatrixExt for Matrix {
    fn rotate_y(angle: f32) -> Matrix {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        
        Matrix {
            m0: cos_angle,  m1: 0.0, m2: -sin_angle, m3: 0.0,
            m4: 0.0,        m5: 1.0, m6: 0.0,        m7: 0.0,
            m8: sin_angle,  m9: 0.0, m10: cos_angle, m11: 0.0,
            m12: 0.0,       m13: 0.0, m14: 0.0,       m15: 1.0,
        }
    }

    fn rotate(axis: Vector3, angle: f32) -> Matrix {
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        
        Matrix {
            m0: t * x * x + c,      m1: t * x * y + s * z,  m2: t * x * z - s * y,  m3: 0.0,
            m4: t * x * y - s * z,  m5: t * y * y + c,      m6: t * y * z + s * x,  m7: 0.0,
            m8: t * x * z + s * y,  m9: t * y * z - s * x,  m10: t * z * z + c,     m11: 0.0,
            m12: 0.0,               m13: 0.0,               m14: 0.0,               m15: 1.0,
        }
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}
