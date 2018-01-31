// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! A one-dimensional length, tagged with its units.

use num_traits;

/// A trait for parameters that can be either a `Length<T, U>` or the raw
/// scalar value `T`.
///
/// This makes it possible to write methods that accept both, for example `TypedVector2D::new`
/// works with both strongly typed length arguments and direct scalar values:
///
/// ```
/// use euclid::{Length, TypedVector2D};
/// struct WorldSpace;
/// struct ScreenSpace;
/// type WorldVec2 = TypedVector2D<f32, WorldSpace>;
/// type WorldLength = Length<f32, WorldSpace>;
/// type ScreenLength = Length<f32, ScreenSpace>;
/// // Convenient synatx:
/// let v1 = WorldVec2::new(1.0, 2.0);
/// // Statically checked synatx:
/// let v2 = WorldVec2::new(WorldLength::new(1.0), WorldLength::new(2.0));
/// // This would give a compile time error because units do not match:
/// // let not_good = WorldVec2::new(ScreenLength:new(1.0), ScreenLength::new(2.0));
/// ```
pub trait ValueOrLength<T, U> {
    fn value(self) -> T;
}

impl<T, U> ValueOrLength<T, U> for T {
    #[inline]
    fn value(self) -> T { self }
}


pub trait ValueOrScale<T, Src, Dst> {
    fn value(self) -> T;
}

impl<T, Src, Dst> ValueOrScale<T, Src, Dst> for T {
    #[inline]
    fn value(self) -> T { self }
}


pub trait Zero {
    fn zero() -> Self;
}

impl<T: num_traits::Zero> Zero for T {
    fn zero() -> T {
        num_traits::Zero::zero()
    }
}

pub trait One {
    fn one() -> Self;
}

impl<T: num_traits::One> One for T {
    fn one() -> T {
        num_traits::One::one()
    }
}

pub trait Round: Copy {
    fn round(self) -> Self;
}
pub trait Floor: Copy {
    fn floor(self) -> Self;
}
pub trait Ceil: Copy {
    fn ceil(self) -> Self;
}

macro_rules! num_int {
    ($ty:ty) => (
        impl Round for $ty {
            #[inline]
            fn round(self) -> $ty { self }
        }
        impl Floor for $ty {
            #[inline]
            fn floor(self) -> $ty { self }
        }
        impl Ceil for $ty {
            #[inline]
            fn ceil(self) -> $ty { self }
        }
    )
}
macro_rules! num_float {
    ($ty:ty) => (
        impl Round for $ty {
            #[inline]
            fn round(self) -> $ty { self.round() }
        }
        impl Floor for $ty {
            #[inline]
            fn floor(self) -> $ty { self.floor() }
        }
        impl Ceil for $ty {
            #[inline]
            fn ceil(self) -> $ty { self.ceil() }
        }
    )
}

num_int!(i16);
num_int!(u16);
num_int!(i32);
num_int!(u32);
num_int!(i64);
num_int!(u64);
num_int!(isize);
num_int!(usize);
num_float!(f32);
num_float!(f64);
