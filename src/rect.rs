// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use length::Length;

use point::Point2D;
use size::Size2D;
use std::cmp::{PartialEq, PartialOrd};
use std::fmt;
use std::num::{NumCast, Zero};

#[deriving(Clone, Decodable, Encodable, PartialEq)]
pub struct Rect<T> {
    pub origin: Point2D<T>,
    pub size: Size2D<T>,
}

impl<T: fmt::Show> fmt::Show for Rect<T> {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect({} at {})", self.size, self.origin)
    }
}

pub fn Rect<T:Clone>(origin: Point2D<T>, size: Size2D<T>) -> Rect<T> {
    Rect {
        origin: origin,
        size: size
    }
}

impl<T: Clone + PartialOrd + Add<T,T> + Sub<T,T>> Rect<T> {
    #[inline]
    pub fn intersects(&self, other: &Rect<T>) -> bool {
        self.origin.x < other.origin.x + other.size.width &&
       other.origin.x <  self.origin.x + self.size.width &&
        self.origin.y < other.origin.y + other.size.height &&
       other.origin.y <  self.origin.y + self.size.height
    }

    #[inline]
    pub fn max_x(&self) -> T {
        self.origin.x + self.size.width
    }

    #[inline]
    pub fn min_x(&self) -> T {
        self.origin.x.clone()
    }

    #[inline]
    pub fn max_y(&self) -> T {
        self.origin.y + self.size.height
    }

    #[inline]
    pub fn min_y(&self) -> T {
        self.origin.y.clone()
    }

    #[inline]
    pub fn max_point(&self) -> Point2D<T> {
        Point2D(self.max_x(), self.max_y())
    }

    #[inline]
    pub fn intersection(&self, other: &Rect<T>) -> Option<Rect<T>> {
        if !self.intersects(other) {
            return None;
        }

        let upper_left = Point2D(max(self.min_x(), other.min_x()),
                                 max(self.min_y(), other.min_y()));

        let lower_right = Point2D(min(self.max_x(), other.max_x()),
                                  min(self.max_y(), other.max_y()));

        Some(Rect(upper_left.clone(), Size2D(lower_right.x - upper_left.x,
                                             lower_right.y - upper_left.y)))
    }

    #[inline]
    pub fn union(&self, other: &Rect<T>) -> Rect<T> {
        let upper_left = Point2D(min(self.min_x(), other.min_x()),
                                 min(self.min_y(), other.min_y()));

        let lower_right = Point2D(max(self.max_x(), other.max_x()),
                                  max(self.max_y(), other.max_y()));

        Rect {
            origin: upper_left.clone(),
            size: Size2D(lower_right.x - upper_left.x, lower_right.y - upper_left.y)
        }
    }

    #[inline]
    pub fn translate(&self, other: &Point2D<T>) -> Rect<T> {
        Rect {
            origin: Point2D(self.origin.x + other.x, self.origin.y + other.y),
            size: self.size.clone()
        }
    }

    #[inline]
    pub fn contains(&self, other: &Point2D<T>) -> bool {
        self.origin.x <= other.x && other.x < self.origin.x + self.size.width &&
        self.origin.y <= other.y && other.y < self.origin.y + self.size.height
    }

    #[inline]
    pub fn inflate(&self, width: T, height: T) -> Rect<T> {
        Rect {
            origin: Point2D(self.origin.x - width, self.origin.y - height),
            size: Size2D(self.size.width + width + width, self.size.height + height + height),
        }
    }
}

impl<Scale, T: Clone + Mul<Scale,T>> Rect<T> {
    #[inline]
    pub fn scale(&self, x: Scale, y: Scale) -> Rect<T> {
        Rect {
            origin: Point2D { x: self.origin.x * x, y: self.origin.y * y},
            size: Size2D { width: self.size.width * x, height: self.size.height * y}
        }
    }
}

impl<T:Clone + Zero> Rect<T> {
    pub fn zero() -> Rect<T> {
        Rect {
            origin: Zero::zero(),
            size: Size2D::zero(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}


pub fn min<T:Clone + PartialOrd>(x: T, y: T) -> T {
    if x <= y { x } else { y }
}

pub fn max<T:Clone + PartialOrd>(x: T, y: T) -> T {
    if x >= y { x } else { y }
}

impl<Scale, T0: Mul<Scale, T1>, T1: Clone> Mul<Scale, Rect<T1>> for Rect<T0> {
    #[inline]
    fn mul(&self, scale: &Scale) -> Rect<T1> {
        Rect(self.origin * *scale, self.size * *scale)
    }
}

impl<Scale, T0: Div<Scale, T1>, T1: Clone> Div<Scale, Rect<T1>> for Rect<T0> {
    #[inline]
    fn div(&self, scale: &Scale) -> Rect<T1> {
        Rect(self.origin / *scale, self.size / *scale)
    }
}

// Convenient aliases for Rect with typed units
pub type TypedRect<Unit, T> = Rect<Length<Unit, T>>;

impl<Unit, T: Clone> Rect<Length<Unit, T>> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Rect<T> {
        Rect(self.origin.to_untyped(), self.size.to_untyped())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(r: &Rect<T>) -> TypedRect<Unit, T> {
        Rect(Point2D::from_untyped(&r.origin), Size2D::from_untyped(&r.size))
    }
}

impl<Unit, T0: NumCast + Clone, T1: NumCast + Clone> Rect<Length<Unit, T0>> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast(&self) -> Option<Rect<Length<Unit, T1>>> {
        match (self.origin.cast(), self.size.cast()) {
            (Some(origin), Some(size)) => Some(Rect(origin, size)),
            _ => None
        }
    }
}

// Convenience functions for common casts
impl<Unit, T: NumCast + Clone> Rect<Length<Unit, T>> {
    pub fn as_f32(&self) -> Rect<Length<Unit, f32>> {
        self.cast().unwrap()
    }

    pub fn as_uint(&self) -> Rect<Length<Unit, uint>> {
        self.cast().unwrap()
    }
}

#[test]
fn test_min_max() {
    assert!(min(0u32, 1u32) == 0u32);
    assert!(min(-1.0f32, 0.0f32) == -1.0f32);

    assert!(max(0u32, 1u32) == 1u32);
    assert!(max(-1.0f32, 0.0f32) == 0.0f32);
}

#[test]
fn test_translate() {
    let p = Rect(Point2D(0u32, 0u32), Size2D(50u32, 40u32));
    let pp = p.translate(&Point2D(10,15));

    assert!(pp.size.width == 50);
    assert!(pp.size.height == 40);
    assert!(pp.origin.x == 10);
    assert!(pp.origin.y == 15);


    let r = Rect(Point2D(-10i32, -5i32), Size2D(50i32, 40i32));
    let rr = r.translate(&Point2D(0,-10));

    assert!(rr.size.width == 50);
    assert!(rr.size.height == 40);
    assert!(rr.origin.x == -10);
    assert!(rr.origin.y == -15);
}

#[test]
fn test_union() {
    let p = Rect(Point2D(0i32, 0i32), Size2D(50i32, 40i32));
    let q = Rect(Point2D(20i32 ,20i32), Size2D(5i32, 5i32));
    let r = Rect(Point2D(-15i32, -30i32), Size2D(200i32, 15i32));
    let s = Rect(Point2D(20i32, -15i32), Size2D(250i32, 200i32));

    let pq = p.union(&q);
    assert!(pq.origin == Point2D(0, 0));
    assert!(pq.size == Size2D(50, 40));

    let pr = p.union(&r);
    assert!(pr.origin == Point2D(-15, -30));
    assert!(pr.size == Size2D(200, 70));

    let ps = p.union(&s);
    assert!(ps.origin == Point2D(0, -15));
    assert!(ps.size == Size2D(270, 200));

}

#[test]
fn test_intersection() {
    let p = Rect(Point2D(0i32, 0i32), Size2D(10i32, 20i32));
    let q = Rect(Point2D(5i32, 15i32), Size2D(10i32, 10i32));
    let r = Rect(Point2D(-5i32, -5i32), Size2D(8i32, 8i32));

    let pq = p.intersection(&q);
    assert!(pq.is_some());
    let pq = pq.unwrap();
    assert!(pq.origin == Point2D(5, 15));
    assert!(pq.size == Size2D(5, 5));

    let pr = p.intersection(&r);
    assert!(pr.is_some());
    let pr = pr.unwrap();
    assert!(pr.origin == Point2D(0, 0));
    assert!(pr.size == Size2D(3, 3));

    let qr = q.intersection(&r);
    assert!(qr.is_none());
}

#[test]
fn test_contains() {
    let r = Rect(Point2D(-20i32, 15i32), Size2D(100i32, 200i32));

    assert!(r.contains(&Point2D(0, 50)));
    assert!(r.contains(&Point2D(-10, 200)));

    // The `contains` method is inclusive of the top/left edges, but not the
    // bottom/right edges.
    assert!(r.contains(&Point2D(-20, 15)));
    assert!(!r.contains(&Point2D(80, 15)));
    assert!(!r.contains(&Point2D(80, 215)));
    assert!(!r.contains(&Point2D(-20, 215)));

    // Points beyond the top-left corner.
    assert!(!r.contains(&Point2D(-25, 15)));
    assert!(!r.contains(&Point2D(-15, 10)));

    // Points beyond the top-right corner.
    assert!(!r.contains(&Point2D(85, 20)));
    assert!(!r.contains(&Point2D(75, 10)));

    // Points beyond the bottom-right corner.
    assert!(!r.contains(&Point2D(85, 210)));
    assert!(!r.contains(&Point2D(75, 220)));

    // Points beyond the bottom-left corner.
    assert!(!r.contains(&Point2D(-25, 210)));
    assert!(!r.contains(&Point2D(-15, 220)));
}

#[test]
fn test_scale() {
    let p = Rect(Point2D(0u32, 0u32), Size2D(50u32, 40u32));
    let pp = p.scale(10, 15);

    assert!(pp.size.width == 500);
    assert!(pp.size.height == 600);
    assert!(pp.origin.x == 0);
    assert!(pp.origin.y == 0);

    let r = Rect(Point2D(-10i32, -5i32), Size2D(50i32, 40i32));
    let rr = r.scale(1, 20);

    assert!(rr.size.width == 50);
    assert!(rr.size.height == 800);
    assert!(rr.origin.x == -10);
    assert!(rr.origin.y == -100);
}

#[test]
fn test_inflate() {
    let p = Rect(Point2D(0i32, 0i32), Size2D(10i32, 10i32));
    let pp = p.inflate(10, 20);

    assert!(pp.size.width == 30);
    assert!(pp.size.height == 50);
    assert!(pp.origin.x == -10);
    assert!(pp.origin.y == -20);

    let r = Rect(Point2D(0i32, 0i32), Size2D(10i32, 20i32));
    let rr = r.inflate(-2, -5);

    assert!(rr.size.width == 6);
    assert!(rr.size.height == 10);
    assert!(rr.origin.x == 2);
    assert!(rr.origin.y == 5);
}

#[test]
fn test_min_max_x_y() {
    let p = Rect(Point2D(0u32, 0u32), Size2D(50u32, 40u32));
    assert!(p.max_y() == 40);
    assert!(p.min_y() == 0);
    assert!(p.max_x() == 50);
    assert!(p.min_x() == 0);

    let r = Rect(Point2D(-10i32, -5i32), Size2D(50i32, 40i32));
    assert!(r.max_y() == 35);
    assert!(r.min_y() == -5);
    assert!(r.max_x() == 40);
    assert!(r.min_x() == -10);
}
