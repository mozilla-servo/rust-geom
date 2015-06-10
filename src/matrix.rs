// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use approxeq::ApproxEq;
use point::Point2D;
use num::{One, Zero};

use num_lib::{Float, NumCast};

pub fn Matrix4<T: Float>(
        m11: T, m12: T, m13: T, m14: T,
        m21: T, m22: T, m23: T, m24: T,
        m31: T, m32: T, m33: T, m34: T,
        m41: T, m42: T, m43: T, m44: T)
     -> Matrix4<T> {
    Matrix4 {
        m11: m11, m12: m12, m13: m13, m14: m14,
        m21: m21, m22: m22, m23: m23, m24: m24,
        m31: m31, m32: m32, m33: m33, m34: m34,
        m41: m41, m42: m42, m43: m43, m44: m44
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix4<T> {
    pub m11: T, pub m12: T, pub m13: T, pub m14: T,
    pub m21: T, pub m22: T, pub m23: T, pub m24: T,
    pub m31: T, pub m32: T, pub m33: T, pub m34: T,
    pub m41: T, pub m42: T, pub m43: T, pub m44: T,
}

impl<T: Zero +
        One +
        ApproxEq<T> +
        Float> Matrix4<T> {
    pub fn approx_eq(&self, other: &Matrix4<T>) -> bool {
        self.m11.approx_eq(&other.m11) && self.m12.approx_eq(&other.m12) &&
        self.m13.approx_eq(&other.m13) && self.m14.approx_eq(&other.m14) &&
        self.m21.approx_eq(&other.m21) && self.m22.approx_eq(&other.m22) &&
        self.m23.approx_eq(&other.m23) && self.m24.approx_eq(&other.m24) &&
        self.m31.approx_eq(&other.m31) && self.m32.approx_eq(&other.m32) &&
        self.m33.approx_eq(&other.m33) && self.m34.approx_eq(&other.m34) &&
        self.m41.approx_eq(&other.m41) && self.m42.approx_eq(&other.m42) &&
        self.m43.approx_eq(&other.m43) && self.m44.approx_eq(&other.m44)
    }

    pub fn mul(&self, m: &Matrix4<T>) -> Matrix4<T> {
        Matrix4(m.m11*self.m11 + m.m12*self.m21 + m.m13*self.m31 + m.m14*self.m41,
                m.m11*self.m12 + m.m12*self.m22 + m.m13*self.m32 + m.m14*self.m42,
                m.m11*self.m13 + m.m12*self.m23 + m.m13*self.m33 + m.m14*self.m43,
                m.m11*self.m14 + m.m12*self.m24 + m.m13*self.m34 + m.m14*self.m44,
                m.m21*self.m11 + m.m22*self.m21 + m.m23*self.m31 + m.m24*self.m41,
                m.m21*self.m12 + m.m22*self.m22 + m.m23*self.m32 + m.m24*self.m42,
                m.m21*self.m13 + m.m22*self.m23 + m.m23*self.m33 + m.m24*self.m43,
                m.m21*self.m14 + m.m22*self.m24 + m.m23*self.m34 + m.m24*self.m44,
                m.m31*self.m11 + m.m32*self.m21 + m.m33*self.m31 + m.m34*self.m41,
                m.m31*self.m12 + m.m32*self.m22 + m.m33*self.m32 + m.m34*self.m42,
                m.m31*self.m13 + m.m32*self.m23 + m.m33*self.m33 + m.m34*self.m43,
                m.m31*self.m14 + m.m32*self.m24 + m.m33*self.m34 + m.m34*self.m44,
                m.m41*self.m11 + m.m42*self.m21 + m.m43*self.m31 + m.m44*self.m41,
                m.m41*self.m12 + m.m42*self.m22 + m.m43*self.m32 + m.m44*self.m42,
                m.m41*self.m13 + m.m42*self.m23 + m.m43*self.m33 + m.m44*self.m43,
                m.m41*self.m14 + m.m42*self.m24 + m.m43*self.m34 + m.m44*self.m44)
    }

    pub fn mul_s(&self, x: T) -> Matrix4<T> {
        Matrix4(self.m11 * x, self.m12 * x, self.m13 * x, self.m14 * x,
                self.m21 * x, self.m22 * x, self.m23 * x, self.m24 * x,
                self.m31 * x, self.m32 * x, self.m33 * x, self.m34 * x,
                self.m41 * x, self.m42 * x, self.m43 * x, self.m44 * x)
    }

    pub fn scale(&self, x: T, y: T, z: T) -> Matrix4<T> {
        Matrix4(self.m11 * x,     self.m12.clone(), self.m13.clone(), self.m14.clone(),
                self.m21.clone(), self.m22 * y,     self.m23.clone(), self.m24.clone(),
                self.m31.clone(), self.m32.clone(), self.m33 * z,     self.m34.clone(),
                self.m41.clone(), self.m42.clone(), self.m43.clone(), self.m44.clone())
    }

    /// Returns the given point transformed by this matrix.
    #[inline]
    pub fn transform_point(&self, p: &Point2D<T>) -> Point2D<T> {
        Point2D(p.x * self.m11 + p.y * self.m21 + self.m41,
                p.x * self.m12 + p.y * self.m22 + self.m42)
    }

    pub fn to_array(&self) -> [T; 16] {
        [
            self.m11.clone(), self.m12.clone(), self.m13.clone(), self.m14.clone(),
            self.m21.clone(), self.m22.clone(), self.m23.clone(), self.m24.clone(),
            self.m31.clone(), self.m32.clone(), self.m33.clone(), self.m34.clone(),
            self.m41.clone(), self.m42.clone(), self.m43.clone(), self.m44.clone()
        ]
    }

    pub fn translate(&self, x: T, y: T, z: T) -> Matrix4<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let matrix = Matrix4(_1.clone(), _0.clone(), _0.clone(), _0.clone(),
                             _0.clone(), _1.clone(), _0.clone(), _0.clone(),
                             _0.clone(), _0.clone(), _1.clone(), _0.clone(),
                             x,  y,  z,  _1.clone());

        return self.mul(&matrix);
    }

    /// Create a 3d translation matrix
    pub fn create_translation(x: T, y: T, z: T) -> Matrix4<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        Matrix4(_1.clone(), _0.clone(), _0.clone(), _0.clone(),
                _0.clone(), _1.clone(), _0.clone(), _0.clone(),
                _0.clone(), _0.clone(), _1.clone(), _0.clone(),
                         x,          y,          z, _1.clone())
    }

    /// Create a 3d scale matrix
    pub fn create_scale(x: T, y: T, z: T) -> Matrix4<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        Matrix4(         x, _0.clone(), _0.clone(), _0.clone(),
                _0.clone(),          y, _0.clone(), _0.clone(),
                _0.clone(), _0.clone(),          z, _0.clone(),
                _0.clone(), _0.clone(), _0.clone(), _1.clone())
    }

    /// Create a 3d rotation matrix from an angle / axis.
    /// The supplied axis must be normalized.
    pub fn create_rotation(x: T, y: T, z: T, theta: T) -> Matrix4<T> {
        let _0: T = Zero::zero();
        let _1: T = One::one();
        let _2: T = NumCast::from(2).unwrap();
        let _half: T = NumCast::from(0.5).unwrap();

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let half_theta = theta * _half;
        let sc = half_theta.sin() * half_theta.cos();
        let sq = half_theta.sin() * half_theta.sin();

        Matrix4(
            _1.clone() - _2.clone() * (yy + zz) * sq,
            _2.clone() * (x * y * sq - z * sc),
            _2.clone() * (x * z * sq + y * sc),
            _0.clone(),

            _2.clone() * (x * y * sq + z * sc),
            _1.clone() - _2.clone() * (xx + zz) * sq,
            _2.clone() * (y * z * sq - x * sc),
            _0.clone(),

            _2.clone() * (x * z * sq - y * sc),
            _2.clone() * (y * z * sq + x * sc),
            _1.clone() - _2.clone() * (xx + yy) * sq,
            _0.clone(),

            _0.clone(),
            _0.clone(),
            _0.clone(),
            _1.clone()
        )
    }

    /// Create a 2d skew matrix
    /// http://dev.w3.org/csswg/css-transforms/#SkewDefined
    pub fn create_skew(alpha: T, beta: T) -> Matrix4<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        Matrix4(_1.clone(), beta.tan(), _0.clone(), _0.clone(),
               alpha.tan(), _1.clone(), _0.clone(), _0.clone(),
                _0.clone(), _0.clone(), _1.clone(), _0.clone(),
                _0.clone(), _0.clone(), _0.clone(), _1.clone())
    }

    /// Create a simple perspective projection matrix
    pub fn create_perspective(d: T) -> Matrix4<T> {
        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        Matrix4(_1.clone(), _0.clone(), _0.clone(), _0.clone(),
                _0.clone(), _1.clone(), _0.clone(), _0.clone(),
                _0.clone(), _0.clone(), _1.clone(), -_1.clone() / d,
                _0.clone(), _0.clone(), _0.clone(), _1.clone())
    }
}

// TODO(gw): Move ortho and identity into static functions of the Matrix type.
pub fn ortho<T: Zero + One + Float>
        (left: T,
         right: T,
         bottom: T,
         top: T,
         near: T,
         far: T)
      -> Matrix4<T> {
    let _2: T = NumCast::from(2).unwrap();
    let _1: T = One::one();
    let _0: T = Zero::zero();

    let tx = -((right + left) / (right - left));
    let ty = -((top + bottom) / (top - bottom));
    let tz = -((far + near) / (far - near));

    Matrix4(_2 / (right - left), _0.clone(),          _0.clone(),         _0.clone(),
            _0.clone(),          _2 / (top - bottom), _0.clone(),         _0.clone(),
            _0.clone(),          _0.clone(),          -_2 / (far - near), _0.clone(),
            tx,                  ty,                  tz,                 _1.clone())
}

pub fn identity<T: Zero + One + Float>() -> Matrix4<T> {
    let (_0, _1): (T, T) = (Zero::zero(), One::one());
    Matrix4(_1.clone(), _0.clone(), _0.clone(), _0.clone(),
            _0.clone(), _1.clone(), _0.clone(), _0.clone(),
            _0.clone(), _0.clone(), _1.clone(), _0.clone(),
            _0.clone(), _0.clone(), _0.clone(), _1.clone())
}

#[test]
pub fn test_ortho() {
    let (left, right, bottom, top) = (0.0f32, 1.0f32, 0.1f32, 1.0f32);
    let (near, far) = (-1.0f32, 1.0f32);
    let result = ortho(left, right, bottom, top, near, far);
    let expected = Matrix4(2.0,  0.0,         0.0,  0.0,
                           0.0,  2.22222222,  0.0,  0.0,
                           0.0,  0.0,         -1.0, 0.0,
                           -1.0, -1.22222222, -0.0, 1.0);
    debug!("result={:?} expected={:?}", result, expected);
    assert!(result.approx_eq(&expected));
}

