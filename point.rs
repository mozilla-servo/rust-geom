// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

#[deriving(Clone, Decodable, Default, Encodable, Eq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T
}

impl<T: fmt::Show> fmt::Show for Point2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f.buf, "({},{})", self.x, self.y)
    }
}

pub fn Point2D<T>(x: T, y: T) -> Point2D<T> {
    Point2D {x: x, y: y}
}


impl<T:Add<T,T>> Add<Point2D<T>, Point2D<T>> for Point2D<T> {
    fn add(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x + other.x, self.y + other.y)
    }
}

impl<T:Clone + Sub<T,T>> Sub<Point2D<T>, Point2D<T>> for Point2D<T> {
    fn sub(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x - other.x, self.y - other.y)
    }
}

